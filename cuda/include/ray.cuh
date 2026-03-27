#ifndef CUDA_RAY_CUH
#define CUDA_RAY_CUH

#include "vec3.cuh"

struct Ray {
    Point3 origin;
    Vec3 direction;
    float time;

    __host__ __device__
    Ray() : origin(), direction(), time(0.0f) {}

    __host__ __device__
    Ray(const Point3& origin, const Vec3& direction, float time = 0.0f)
        : origin(origin), direction(direction), time(time) {}

    __host__ __device__
    Point3 at(float t) const {
        return origin + direction * t;
    }
};

#endif // CUDA_RAY_CUH