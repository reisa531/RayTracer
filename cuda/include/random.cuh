#ifndef CUDA_RANDOM_CUH
#define CUDA_RANDOM_CUH

#include <curand_kernel.h>

#include "vec3.cuh"

__device__
float random_float(curandState* state) {
    return curand_uniform(state);
}

__device__
float random_float(curandState* state, float min, float max) {
    return min + (max - min) * random_float(state);
}

__device__
int random_int(curandState* state, int min, int max) {
    return min + curand(state) % (max - min + 1);
}

__device__
Vec3 random_vec3(curandState* state, float min, float max) {
    return Vec3(random_float(state, min, max),
                random_float(state, min, max),
                random_float(state, min, max));
}

__device__
Vec3 random_unit(curandState* state) {
    float a = random_float(state, 0, 2 * M_PI);
    float z = random_float(state, -1, 1);
    float r = sqrtf(1 - z * z);
    return Vec3(r * cosf(a), r * sinf(a), z);
}

__device__
Vec3 random_in_unit_disk(curandState* state) {
    float theta = random_float(state, 0, 2 * M_PI);
    float radius = sqrtf(random_float(state));
    return Vec3(radius * cosf(theta), radius * sinf(theta), 0);
}

__device__
Vec3 random_unit_on_hemisphere(curandState* state, const Vec3& normal) {
    Vec3 in_unit_sphere = random_unit(state);
    if (in_unit_sphere.dot(normal) > 0.0f) {
        return in_unit_sphere;
    } else {
        return in_unit_sphere * -1.0f;
    }
}

#endif // CUDA_RANDOM_CUH