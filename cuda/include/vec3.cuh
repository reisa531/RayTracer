#ifndef CUDA_VEC3_CUH
#define CUDA_VEC3_CUH

#include <stdio.h>

struct Vec3 {
    float x, y, z;

    __host__ __device__
    Vec3() : x(0), y(0), z(0) {}

    __host__ __device__
    Vec3(float x, float y, float z) : x(x), y(y), z(z) {}

    __host__ __device__
    Vec3 operator+(const Vec3& other) const {
        return Vec3(x + other.x, y + other.y, z + other.z);
    }

    __host__ __device__
    Vec3 operator-(const Vec3& other) const {
        return Vec3(x - other.x, y - other.y, z - other.z);
    }

    __host__ __device__
    Vec3 operator*(float scalar) const {
        return Vec3(x * scalar, y * scalar, z * scalar);
    }

    __host__ __device__
    Vec3 operator/(float scalar) const {
        return Vec3(x / scalar, y / scalar, z / scalar);
    }

    __host__ __device__
    float dot(const Vec3& other) const {
        return x * other.x + y * other.y + z * other.z;
    }

    __host__ __device__
    Vec3 cross(const Vec3& other) const {
        return Vec3(
            y * other.z - z * other.y,
            z * other.x - x * other.z,
            x * other.y - y * other.x
        );
    }

    __host__ __device__
    float length() const {
        return sqrtf(x * x + y * y + z * z);
    }

    __host__ __device__
    float length_squared() const {
        return x * x + y * y + z * z;
    }

    __host__ __device__
    Vec3 normalize() const {
        float len = length();
        if (len > 0) {
            return (*this) / len;
        }
        return Vec3(0, 0, 0);
    }

    __host__ __device__
    float operator[](int index) const {
        if (index == 0) return x;
        if (index == 1) return y;
        if (index == 2) return z;
        return 0.0f;
    }

    __host__ __device__
    float& operator[](int index) {
        if (index == 0) return x;
        if (index == 1) return y;
        if (index == 2) return z;
        static float dummy = 0.0f;
        return dummy;
    }

    __host__ __device__
    Vec3 hadamard_product(const Vec3& other) const {
        return Vec3(x * other.x, y * other.y, z * other.z);
    }

    __host__ __device__
    Vec3 operator-() const {
        return Vec3(-x, -y, -z);
    }

    __host__ __device__
    Vec3 operator+=(const Vec3& other) {
        if (this == &other) {
            return *this;
        }
        x += other.x;
        y += other.y;
        z += other.z;
        return *this;
    }

    __host__ __device__
    Vec3 operator-=(const Vec3& other) {
        if (this == &other) {
            return *this;
        }
        x -= other.x;
        y -= other.y;
        z -= other.z;
        return *this;
    }

    __host__ __device__
    Vec3 operator*=(float scalar) {
        x *= scalar;
        y *= scalar;
        z *= scalar;
        return *this;
    }

    __host__ __device__
    Vec3 operator/=(float scalar) {
        if (scalar == 0) {
            return *this;
        }
        x /= scalar;
        y /= scalar;
        z /= scalar;
        return *this;
    }

    __host__ __device__
    bool near_zero() const {
        const float s = 1e-8f;
        return (fabsf(x) < s) && (fabsf(y) < s) && (fabsf(z) < s);
    }

    __host__ __device__
    Vec3 reflect(const Vec3& normal) const {
        return (*this) - normal * (2.0f * this->dot(normal));
    }

    __host__ __device__
    Vec3 refract(const Vec3& normal, float refractive_index) const {
        Vec3 uv = *this;
        Vec3 neg_uv = uv * -1.0f;
        float cos_theta = fminf(neg_uv.dot(normal), 1.0f);
        Vec3 r_out_perp = (uv + normal * cos_theta) * refractive_index;
        float k = 1.0f - r_out_perp.length_squared();
        Vec3 r_out_parallel = Vec3(0.0f, 0.0f, 0.0f);
        if (k > 0.0f) {
            r_out_parallel = normal * (-sqrtf(k));
        }
        return r_out_perp + r_out_parallel;
    }

    __host__ __device__
    void print_color() const {
        int ir = static_cast<int>(255.999f * x);
        int ig = static_cast<int>(255.999f * y);
        int ib = static_cast<int>(255.999f * z);
        printf("%d %d %d\n", ir, ig, ib);
    }
};

typedef Vec3 Point3;
typedef Vec3 Color;

#endif // CUDA_VEC3_CUH