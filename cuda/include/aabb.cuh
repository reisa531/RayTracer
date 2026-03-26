#ifndef CUDA_AABB_CUH
#define CUDA_AABB_CUH

#include "vec3.cuh"
#include "interval.cuh"
#include "ray.cuh"

struct AABB {
    Interval x;
    Interval y;
    Interval z;

    __host__ __device__
    AABB(const Interval& x, const Interval& y, const Interval& z) : x(x), y(y), z(z) {}

    __host__ __device__
    AABB() : x(0, 0), y(0, 0), z(0, 0) {}

    __host__ __device__
    AABB(const Point3& min, const Point3& max) : x(min.x, max.x), y(min.y, max.y), z(min.z, max.z) {}

    __host__ __device__
    AABB(const AABB& a, const AABB& b) {
        x = Interval(a.x, b.x);
        y = Interval(a.y, b.y);
        z = Interval(a.z, b.z);
    }

    __host__ __device__
    AABB(const Point3& a, const Point3& b, const Point3& c) {
        float xmin = fminf(a.x, fminf(b.x, c.x));
        float xmax = fmaxf(a.x, fmaxf(b.x, c.x));
        float ymin = fminf(a.y, fminf(b.y, c.y));
        float ymax = fmaxf(a.y, fmaxf(b.y, c.y));
        float zmin = fminf(a.z, fminf(b.z, c.z));
        float zmax = fmaxf(a.z, fmaxf(b.z, c.z));

        x = Interval(xmin, xmax);
        y = Interval(ymin, ymax);
        z = Interval(zmin, zmax);

        if (x.size() < 1e-6f) {
            x.expand(1e-6f);
        }
        if (y.size() < 1e-6f) {
            y.expand(1e-6f);
        }
        if (z.size() < 1e-6f) {
            z.expand(1e-6f);
        }
    }

    __host__ __device__
    AABB from_triangle(const Point3& a, const Point3& b, const Point3& c) {
        return AABB(a, b, c);
    }

    __host__ __device__
    AABB from_quad(const Point3& q, const Vec3& u, const Vec3& v) {
        Point3 a = q;
        Point3 b = q + u;
        Point3 c = q + v;
        Point3 d = q + u + v;
        
        float xmin = fminf(a.x, fminf(b.x, fminf(c.x, d.x)));
        float xmax = fmaxf(a.x, fmaxf(b.x, fmaxf(c.x, d.x)));
        float ymin = fminf(a.y, fminf(b.y, fminf(c.y, d.y)));
        float ymax = fmaxf(a.y, fmaxf(b.y, fmaxf(c.y, d.y)));
        float zmin = fminf(a.z, fminf(b.z, fminf(c.z, d.z)));
        float zmax = fmaxf(a.z, fmaxf(b.z, fmaxf(c.z, d.z)));

        Interval x(xmin, xmax);
        Interval y(ymin, ymax);
        Interval z(zmin, zmax);

        if (x.size() < 1e-6f) {
            x.expand(1e-6f);
        }
        if (y.size() < 1e-6f) {
            y.expand(1e-6f);
        }
        if (z.size() < 1e-6f) {
            z.expand(1e-6f);
        }

        return AABB(x, y, z);
    }

    __host__ __device__
    const Interval& axis_interval(int axis) const {
        if (axis == 0) {
            return x;
        } else if (axis == 1) {
            return y;
        } else {
            return z;
        }
    }

    __host__ __device__
    bool hit(const Ray& ray, const Interval& ray_t) const {
        const Point3& ray_origin = ray.origin;
        const Vec3& ray_direction = ray.direction;
        Interval result_interval = ray_t;

        for (int i = 0; i < 3; i++) {
            float invD = 1.0f / ray_direction[i];
            float t0 = (axis_interval(i).min - ray_origin[i]) * invD;
            float t1 = (axis_interval(i).max - ray_origin[i]) * invD;

            if (t0 < t1) {
                if (t0 > result_interval.min) {
                    result_interval.min = t0;
                }
                if (t1 < result_interval.max) {
                    result_interval.max = t1;
                }
            }
            else {
                if (t1 > result_interval.min) {
                    result_interval.min = t1;
                }
                if (t0 < result_interval.max) {
                    result_interval.max = t0;
                }
            }

            if (result_interval.max <= result_interval.min) {
                return false;
            }
        }
        return true;
    }

    int longest_axis() const {
        float x_size = x.size();
        float y_size = y.size();
        float z_size = z.size();

        if (x_size > y_size && x_size > z_size) {
            return 0;
        } else if (y_size > z_size) {
            return 1;
        } else {
            return 2;
        }
    }
};

const AABB EMPTY_AABB(Point3(FLT_MAX, FLT_MAX, FLT_MAX), Point3(-FLT_MAX, -FLT_MAX, -FLT_MAX));
const AABB UNIVERSE_AABB(Point3(-FLT_MAX, -FLT_MAX, -FLT_MAX), Point3(FLT_MAX, FLT_MAX, FLT_MAX));

#endif // CUDA_AABB_CUH