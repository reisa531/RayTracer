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

    bool contains(float t) const {
        return t >= min && t <= max;
    }

    bool surrounds(float t) const {
        return t > min && t < max;
    }

    bool is_valid() const {
        return min < max;
    }

    float size() const {
        return max - min;
    }

    void expand(float t) {
        if (t < min) {
            min = t;
        }
        if (t > max) {
            max = t;
        }
    }

    float clamp(float t) const {
        if (t < min) {
            return min;
        }
        if (t > max) {
            return max;
        }
        return t;
    }

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