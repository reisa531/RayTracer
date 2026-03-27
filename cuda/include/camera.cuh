#ifndef CUDA_CAMERA_CUH
#define CUDA_CAMERA_CUH

#include "material.cuh"
#include "vec3.cuh"
#include "bvh.cuh"
#include "hittable.cuh"
#include "random.cuh"
#include "ray.cuh"
#include <cuda_runtime.h>
#include <curand_kernel.h>

#include <chrono>
#include <cstdio>
#include <thread>
#include <vector>
#include <iostream>

static __host__ inline float clamp01(float x) {
    if (!isfinite(x)) return 0.0f;
    if (x < 0.0f) return 0.0f;
    if (x > 0.999f) return 0.999f;
    return x;
}

static __host__ inline void write_color_ppm(FILE* output, const Color& c) {
    float r = sqrtf(fmaxf(0.0f, c.x));
    float g = sqrtf(fmaxf(0.0f, c.y));
    float b = sqrtf(fmaxf(0.0f, c.z));

    int ir = static_cast<int>(256.0f * clamp01(r));
    int ig = static_cast<int>(256.0f * clamp01(g));
    int ib = static_cast<int>(256.0f * clamp01(b));
    fprintf(output, "%d %d %d\n", ir, ig, ib);
}

struct Camera;

__global__ void init_rand_kernel(curandState *rand_states, int image_width, int image_height, unsigned long long seed);

__global__ void render_kernel(const Camera *camera, BVH *bvh, MaterialList *materials,
                              Color *framebuffer, curandState *rand_states,
                              unsigned long long *progress_counter);

struct Camera {
    float aspect_ratio;
    int image_width;
    int image_height;
    int samples_per_pixel;
    int max_depth;
    float vfov;
    Point3 lookfrom;
    Point3 lookat;
    Vec3 vup;
    float defocus_angle;
    float focus_dist;
    float pixel_sample_scale;
    Point3 center;
    Point3 pixel00_loc;
    Vec3 pixel_delta_u;
    Vec3 pixel_delta_v;
    Vec3 defocus_disk_u;
    Vec3 defocus_disk_v;
    Color background;
    bool use_bvh;
    unsigned long long rng_seed;

    __host__
            Camera(float aspect_ratio, int image_width, int image_height, int samples_per_pixel, int max_depth,
           float vfov, Point3 lookfrom, Point3 lookat, Vec3 vup, float defocus_angle, float focus_dist,
                Color background, bool use_bvh = true, unsigned long long rng_seed = 1337ULL)
        : aspect_ratio(aspect_ratio), image_width(image_width), image_height(image_height),
          samples_per_pixel(samples_per_pixel), max_depth(max_depth), vfov(vfov), lookfrom(lookfrom),
          lookat(lookat), vup(vup), defocus_angle(defocus_angle), focus_dist(focus_dist),
               background(background), use_bvh(use_bvh), rng_seed(rng_seed) {
                this->center = lookfrom;
                this->pixel_sample_scale = 1.0f / samples_per_pixel;

        float theta = vfov * M_PI / 180.0f;
        float h = tan(theta / 2);
                float viewport_height = 2.0f * h * focus_dist;
        float viewport_width = aspect_ratio * viewport_height;

        Vec3 w = (lookfrom - lookat).normalize();
        Vec3 u = vup.cross(w).normalize();
        Vec3 v = w.cross(u);

        Vec3 viewport_u = u * viewport_width;
        Vec3 viewport_v = v * (-viewport_height);

        Vec3 pixel_delta_u = viewport_u / image_width;
        Vec3 pixel_delta_v = viewport_v / image_height;

        Vec3 viewport_upper_left = this->center - w * focus_dist - viewport_u / 2 - viewport_v / 2;

        Vec3 pixel00_loc = viewport_upper_left + pixel_delta_u / 2 + pixel_delta_v / 2;

        float focus_radius = tanf((defocus_angle * M_PI / 180.0f) * 0.5f) * focus_dist;
        Vec3 defocus_disk_u = u * focus_radius;
        Vec3 defocus_disk_v = v * focus_radius;

        this->pixel00_loc = pixel00_loc;
        this->pixel_delta_u = pixel_delta_u;
        this->pixel_delta_v = pixel_delta_v;
        this->defocus_disk_u = defocus_disk_u;
        this->defocus_disk_v = defocus_disk_v;
    }

    __device__
    Vec3 sample_square(curandState *rand_state) const {
        return Vec3(random_float(rand_state) - 0.5f, random_float(rand_state) - 0.5f, 0);
    }

    __device__
    Vec3 sample_defocus_disk(curandState *rand_state) const {
        Vec3 p = random_in_unit_disk(rand_state);
        return center + defocus_disk_u * p.x + defocus_disk_v * p.y;
    }

    __device__
    Ray get_ray(int i, int j, curandState *rand_state) const {
        Vec3 offset = sample_square(rand_state);
        Vec3 pixel_sample = pixel00_loc + pixel_delta_u * (i + offset.x) + pixel_delta_v * (j + offset.y);
        Vec3 ray_origin = defocus_angle <= 0.0 ? center : sample_defocus_disk(rand_state);
        return Ray(ray_origin, pixel_sample - ray_origin, random_float(rand_state));
    }

    __device__
    bool hit_world_linear(const Ray& ray, HittableList *hittables, HitRecord& out_record) const {
        bool any_hit = false;
        float closest = FLT_MAX;
        HitRecord temp;

        for (int i = 0; i < hittables->count; ++i) {
            if (hittables->hit(i, ray, Interval(0.001f, closest), temp)) {
                any_hit = true;
                closest = temp.t;
                out_record = temp;
            }
        }

        return any_hit;
    }

    __device__
    Color ray_color(Ray& ray, BVH *bvh, MaterialList *materials, curandState *rand_state, int depth) const {
        Ray current_ray = ray;
        Color throughput(1.0f, 1.0f, 1.0f);
        Color radiance(0.0f, 0.0f, 0.0f);

        for (int bounce = depth; bounce < max_depth; ++bounce) {
            HitRecord rec;
            bool hit_any = use_bvh
                ? bvh->hit(current_ray, Interval(0.001f, FLT_MAX), rec)
                : hit_world_linear(current_ray, bvh->hittables, rec);

            if (!hit_any) {
                radiance += throughput.hadamard_product(background);
                return radiance;
            }

            Color emitted = materials->emitted(rec.materialId, rec.u, rec.v, rec.p);
            radiance += throughput.hadamard_product(emitted);

            Vec3 attenuation;
            Ray scattered;
            if (!materials->scatter(rec.materialId, current_ray, rec, rand_state, attenuation, scattered)) {
                return radiance;
            }

            throughput = throughput.hadamard_product(attenuation);
            if (!isfinite(throughput.x) || !isfinite(throughput.y) || !isfinite(throughput.z)) {
                return radiance;
            }

            current_ray = scattered;
        }

        return radiance;
    }

    __host__
    void render(FILE* output_path, BVH *bvh, MaterialList *materials) const {
        std::cerr << "use_bvh: " << (use_bvh ? "true" : "false") << std::endl;

        if (output_path == nullptr || bvh == nullptr || materials == nullptr) {
            std::cerr << "render precondition failed: null output/bvh/materials pointer" << std::endl;
            return;
        }

        fprintf(output_path, "P3\n%d %d\n255\n", image_width, image_height);

        int total_pixels = image_width * image_height;
        if (total_pixels <= 0) {
            return;
        }

        Color *d_framebuffer = nullptr;
        curandState *d_rand_states = nullptr;
        unsigned long long *d_progress_counter = nullptr;
        Camera *d_camera = nullptr;
        cudaEvent_t render_done_event = nullptr;
        cudaStream_t render_stream = nullptr;
        cudaStream_t progress_stream = nullptr;

        bool failed = false;
        auto check_cuda = [&](cudaError_t err, const char* op) -> bool {
            if (err == cudaSuccess) {
                return true;
            }
            std::cerr << "CUDA error in " << op << ": " << cudaGetErrorString(err) << std::endl;
            failed = true;
            return false;
        };

        check_cuda(cudaMalloc(&d_framebuffer, sizeof(Color) * total_pixels), "cudaMalloc(d_framebuffer)");
        if (!failed) {
            check_cuda(cudaMalloc(&d_rand_states, sizeof(curandState) * total_pixels), "cudaMalloc(d_rand_states)");
        }
        if (!failed) {
            check_cuda(cudaMalloc(&d_progress_counter, sizeof(unsigned long long)), "cudaMalloc(d_progress_counter)");
        }
        if (!failed) {
            check_cuda(cudaMalloc(&d_camera, sizeof(Camera)), "cudaMalloc(d_camera)");
        }
        if (!failed) {
            check_cuda(cudaMemset(d_progress_counter, 0, sizeof(unsigned long long)), "cudaMemset(d_progress_counter)");
        }
        if (!failed) {
            check_cuda(cudaMemcpy(d_camera, this, sizeof(Camera), cudaMemcpyHostToDevice), "cudaMemcpy(d_camera)");
        }

        dim3 block_dim(16, 16);
        dim3 grid_dim((image_width + block_dim.x - 1) / block_dim.x,
                      (image_height + block_dim.y - 1) / block_dim.y);

        unsigned long long seed = rng_seed == 0ULL
            ? static_cast<unsigned long long>(std::chrono::high_resolution_clock::now().time_since_epoch().count())
            : rng_seed;

        if (!failed) {
            init_rand_kernel<<<grid_dim, block_dim>>>(d_rand_states, image_width, image_height, seed);
            check_cuda(cudaGetLastError(), "init_rand_kernel launch");
        }
        if (!failed) {
            check_cuda(cudaDeviceSynchronize(), "init_rand_kernel sync");
        }

        if (!failed) {
            check_cuda(cudaEventCreate(&render_done_event), "cudaEventCreate(render_done_event)");
        }
        if (!failed) {
            check_cuda(cudaStreamCreateWithFlags(&render_stream, cudaStreamNonBlocking), "cudaStreamCreateWithFlags(render_stream)");
        }
        if (!failed) {
            check_cuda(cudaStreamCreateWithFlags(&progress_stream, cudaStreamNonBlocking), "cudaStreamCreateWithFlags(progress_stream)");
        }

        if (!failed) {
            render_kernel<<<grid_dim, block_dim, 0, render_stream>>>(d_camera, bvh, materials, d_framebuffer,
                           d_rand_states, d_progress_counter);
            check_cuda(cudaGetLastError(), "render_kernel launch");
        }
        if (!failed) {
            check_cuda(cudaEventRecord(render_done_event, render_stream), "cudaEventRecord(render_done_event)");
        }

        const int bar_width = 40;
        unsigned long long progress = 0;
        while (!failed) {
            cudaError_t query_status = cudaEventQuery(render_done_event);
            if (query_status == cudaSuccess) {
                break;
            }
            if (query_status != cudaErrorNotReady) {
                check_cuda(query_status, "cudaEventQuery(render_done_event)");
                break;
            }

            check_cuda(cudaMemcpyAsync(&progress, d_progress_counter, sizeof(unsigned long long),
                            cudaMemcpyDeviceToHost, progress_stream), "cudaMemcpyAsync(progress)");
            if (!failed) {
                check_cuda(cudaStreamSynchronize(progress_stream), "cudaStreamSynchronize(progress_stream)");
            }
            if (failed) {
                break;
            }

            if (progress > static_cast<unsigned long long>(total_pixels)) {
                progress = static_cast<unsigned long long>(total_pixels);
            }
            float ratio = static_cast<float>(progress) / static_cast<float>(total_pixels);
            int filled = static_cast<int>(ratio * bar_width);

            fprintf(stderr, "\r[");
            for (int k = 0; k < bar_width; ++k) {
                fputc(k < filled ? '=' : ' ', stderr);
            }
            fprintf(stderr, "] %6.2f%% (%llu/%d)", ratio * 100.0f, progress, total_pixels);
            fflush(stderr);

            std::this_thread::sleep_for(std::chrono::milliseconds(100));
        }

        if (!failed) {
            check_cuda(cudaEventSynchronize(render_done_event), "cudaEventSynchronize(render_done_event)");
        }

        if (!failed) {
            progress = total_pixels;
            fprintf(stderr, "\r[");
            for (int k = 0; k < bar_width; ++k) {
                fputc('=', stderr);
            }
            fprintf(stderr, "] 100.00%% (%d/%d)\n", total_pixels, total_pixels);
            fflush(stderr);
        }

        if (!failed) {
            std::vector<Color> host_framebuffer(total_pixels);
            check_cuda(cudaMemcpy(host_framebuffer.data(), d_framebuffer, sizeof(Color) * total_pixels, cudaMemcpyDeviceToHost),
                       "cudaMemcpy(framebuffer D2H)");
            if (!failed) {
                for (int j = 0; j < image_height; ++j) {
                    for (int i = 0; i < image_width; ++i) {
                        int idx = j * image_width + i;
                        write_color_ppm(output_path, host_framebuffer[idx]);
                    }
                }
            }
        }

        if (render_done_event != nullptr) {
            cudaEventDestroy(render_done_event);
        }
        if (progress_stream != nullptr) {
            cudaStreamDestroy(progress_stream);
        }
        if (render_stream != nullptr) {
            cudaStreamDestroy(render_stream);
        }
        if (d_camera != nullptr) {
            cudaFree(d_camera);
        }
        if (d_progress_counter != nullptr) {
            cudaFree(d_progress_counter);
        }
        if (d_rand_states != nullptr) {
            cudaFree(d_rand_states);
        }
        if (d_framebuffer != nullptr) {
            cudaFree(d_framebuffer);
        }

        if (failed) {
            std::cerr << "render aborted due to CUDA failure" << std::endl;
        }
    }
};

__global__ void init_rand_kernel(curandState *rand_states, int image_width, int image_height, unsigned long long seed) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    int j = blockIdx.y * blockDim.y + threadIdx.y;
    if (i >= image_width || j >= image_height) {
        return;
    }
    int idx = j * image_width + i;
    curand_init(seed, idx, 0, &rand_states[idx]);
}

__global__ void render_kernel(const Camera *camera, BVH *bvh, MaterialList *materials,
                              Color *framebuffer, curandState *rand_states,
                              unsigned long long *progress_counter) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    int j = blockIdx.y * blockDim.y + threadIdx.y;
    if (i >= camera->image_width || j >= camera->image_height) {
        return;
    }

    int idx = j * camera->image_width + i;
    curandState local_state = rand_states[idx];

    Color pixel_color(0, 0, 0);
    for (int s = 0; s < camera->samples_per_pixel; ++s) {
        Ray ray = camera->get_ray(i, j, &local_state);
        pixel_color += camera->ray_color(ray, bvh, materials, &local_state, 0);
    }
    pixel_color *= camera->pixel_sample_scale;

    framebuffer[idx] = pixel_color;
    rand_states[idx] = local_state;
    atomicAdd(progress_counter, 1ULL);
}

#endif // CUDA_CAMERA_CUH
