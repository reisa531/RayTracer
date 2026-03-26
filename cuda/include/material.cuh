#ifndef CUDA_MATERIAL_CUH
#define CUDA_MATERIAL_CUH

#include "vec3.cuh"
#include "hittable.cuh"
#include "ray.cuh"
#include "random.cuh"
#include "texture.cuh"

#include <curand_kernel.h>

enum MaterialType {
    UnknownMaterial = 0,
    Lambertian,
    Metal,
    Dielectric
};

struct MaterialList {
    int count;
    int *textureId;
    Vec3 *albedo;
    float *constraint1;
    float *constraint2;
    MaterialType *type;
    TextureList *textures;
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
    scattered = Ray(rec.p, scatter_direction);
    attenuation = textures->sample(texture_id, rec.u, rec.v, rec.p);
    return true;
}

__device__
bool scatter_metal(const Vec3& albedo, float fuzz, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered) {
    Vec3 reflected = ray_in.direction.normalize().reflect(rec.normal);
    if (fuzz > 0) {
        Vec3 fuzz_vector = random_unit(rand_state) * fuzz;
        reflected = reflected + fuzz_vector;
    }
    scattered = Ray(rec.p, reflected);
    attenuation = albedo;
    return true;
}

__device__
bool scatter_dielectric(float ref_idx, const Ray& ray_in, const HitRecord& rec, curandState* rand_state, Vec3& attenuation, Ray& scattered) {
    float ri = rec.front_face ? (1.0f / ref_idx) : ref_idx;
    Vec3 unit_direction = ray_in.direction.normalize();
    float cos_theta = fminf((-unit_direction).dot(rec.normal), 1.0f);
    float sin_theta = sqrtf(1.0f - cos_theta * cos_theta);
    if (ri * sin_theta > 1.0f || reflectance(cos_theta, ri) > random_float(rand_state)) {
        Vec3 reflected = unit_direction.reflect(rec.normal);
        scattered = Ray(rec.p, reflected);
        attenuation = Vec3(1.0f, 1.0f, 1.0f);
        return true;
    }
    Vec3 refracted = unit_direction.refract(rec.normal, ri);
    scattered = Ray(rec.p, refracted);
    attenuation = Vec3(1.0f, 1.0f, 1.0f);
    return true;
}

#endif // CUDA_MATERIAL_CUH