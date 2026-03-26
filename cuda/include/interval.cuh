#ifndef CUDA_INTERVAL_CUH
#define CUDA_INTERVAL_CUH

#include <float.h>

struct Interval {
    float min;
    float max;

    __host__ __device__
    Interval() : min(0), max(0) {}

    __host__ __device__
    Interval(float t_min, float t_max) : min(t_min), max(t_max) {}

    __host__ __device__
    Interval(const Interval& a, const Interval& b) {
        min = fminf(a.min, b.min);
        max = fmaxf(a.max, b.max);
    }

    __host__ __device__
    bool contains(float t) const {
        return t >= min && t <= max;
    }

    __host__ __device__
    bool surrounds(float t) const {
        return t > min && t < max;
    }

    __host__ __device__
    bool is_valid() const {
        return min < max;
    }

    __host__ __device__
    float size() const {
        return max - min;
    }

    __host__ __device__
    void expand(float t) {
        if (t < min) {
            min = t;
        }
        if (t > max) {
            max = t;
        }
    }

    __host__ __device__
    float clamp(float t) const {
        if (t < min) {
            return min;
        }
        if (t > max) {
            return max;
        }
        return t;
    }

    __host__ __device__
    Interval operator+(float t) const {
        return Interval(min + t, max + t);
    }
};

const Interval EMPTY_INTERVAL(FLT_MAX, -FLT_MAX);
const Interval UNIVERSE_INTERVAL(-FLT_MAX, FLT_MAX);
const Interval UNIT_INTERVAL(0.0f, 1.0f);
const Interval POSITIVE_INTERVAL(0.0f, FLT_MAX);
const Interval NEGATIVE_INTERVAL(-FLT_MAX, 0.0f);

#endif // CUDA_INTERVAL_CUH