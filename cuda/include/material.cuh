#ifndef CUDA_MATERIAL_CUH
#define CUDA_MATERIAL_CUH

#include "vec3.cuh"
#include "hittable.cuh"
#include "ray.cuh"
#include "random.cuh"
#include "texture.cuh"

#include <cuda_runtime.h>
#include <curand_kernel.h>

enum MaterialType {
    UnknownMaterial = 0,
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
    Isotropic
};

__device__
bool scatter_lambertian(int texture_id, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered, TextureList *textures);

__device__
bool scatter_metal(const Vec3& albedo, float fuzz, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered);

__device__
bool scatter_dielectric(float ref_idx, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered);

struct MaterialList {
    int count;
    int *textureId;
    Vec3 *albedo;
    float *constraint1;
    float *constraint2;
    MaterialType *type;
    TextureList *textures;

    __device__
    bool scatter(int material_id, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered) const {
        MaterialType t = type[material_id];
        switch (t) {
            case Lambertian:
                return scatter_lambertian(textureId[material_id], ray_in, rec, rand_state, attenuation, scattered, textures);
            case Metal:
                return scatter_metal(albedo[material_id], constraint1[material_id], ray_in  , rec, rand_state, attenuation, scattered);
            case Dielectric:
                return scatter_dielectric(constraint1[material_id], ray_in, rec, rand_state , attenuation, scattered);
            case Isotropic:
                attenuation = textures->sample(textureId[material_id], rec.u, rec.v, rec.p);
                scattered = Ray(rec.p, random_unit(rand_state), ray_in.time);
                return true;
            case DiffuseLight:
                return false;
            default:
                return false;
        }
    }

    __device__
    Color emitted(int material_id, float u, float v, const Point3& p) const {
        MaterialType t = type[material_id];
        if (t == DiffuseLight) {
            return textures->sample(textureId[material_id], u, v, p);
        }
        return Color(0.0f, 0.0f, 0.0f);
    }
};

__device__
float reflectance(float cosine, float ref_idx) {
    float r0 = (1.0f - ref_idx) / (1.0f + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0f - r0) * powf((1.0f - cosine), 5);
}

__device__
bool scatter_lambertian(int texture_id, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered, TextureList *textures) {
    Vec3 scatter_direction = rec.normal + random_unit(rand_state);
    if (scatter_direction.near_zero()) {
        scatter_direction = rec.normal;
    }
    scattered = Ray(rec.p, scatter_direction, ray_in.time);
    attenuation = textures->sample(texture_id, rec.u, rec.v, rec.p);
    return true;
}

__device__
bool scatter_metal(const Vec3& albedo, float fuzz, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered) {
    Vec3 reflected = ray_in.direction.normalize().reflect(rec.normal).normalize()
        + random_unit(rand_state) * fmaxf(0.0f, fminf(fuzz, 1.0f));
    scattered = Ray(rec.p, reflected, ray_in.time);
    attenuation = albedo;
    return scattered.direction.dot(rec.normal) > 0.0f;
}

__device__
bool scatter_dielectric(float ref_idx, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered) {
    float ri = rec.front_face ? (1.0f / ref_idx) : ref_idx;
    Vec3 unit_direction = ray_in.direction.normalize();
    float cos_theta = fminf((-unit_direction).dot(rec.normal), 1.0f);
    float sin_theta = sqrtf(fmaxf(0.0f, 1.0f - cos_theta * cos_theta));
    Vec3 direction;
    if (ri * sin_theta > 1.0f || reflectance(cos_theta, ri) > random_float(rand_state)) {
        direction = unit_direction.reflect(rec.normal);
    } else {
        direction = unit_direction.refract(rec.normal, ri);
    }
    scattered = Ray(rec.p, direction, ray_in.time);
    attenuation = Vec3(1.0f, 1.0f, 1.0f);
    return true;
}

#endif // CUDA_MATERIAL_CUH