#ifndef CUDA_HITTABLE_CUH
#define CUDA_HITTABLE_CUH

#include "vec3.cuh"
#include "ray.cuh"
#include "interval.cuh"

enum HittableType {
    UnknownHittable = 0,
    Sphere,
    Triangle,
    Quad
};

struct HitRecord {
    Point3 p;
    Vec3 normal;
    int materialId;
    float t;
    float u;
    float v;
    bool front_face;

    __host__ __device__
    void set_face_normal(const Ray& ray, const Vec3& outward_normal) {
        front_face = ray.direction.dot(outward_normal) < 0;
        normal = front_face ? outward_normal : outward_normal * -1.0f;
    }
};

__host__ __device__
HitRecord *hit_sphere(const Vec3 &center, float radius, const Ray &ray, const Interval& ray_t);

__host__ __device__
HitRecord *hit_triangle(const Vec3& a, const Vec3& edge1, const Vec3& edge2, const Ray& ray, const Interval& ray_t);

__host__ __device__
HitRecord *hit_quad(const Vec3& q, const Vec3& u, const Vec3& v, const Vec3& w, const Ray& ray, const Interval& ray_t);

// SoA style hittable list for GPU ray tracing
struct HittableList {
    int count;
    Vec3 *point;
    Vec3 *u;
    Vec3 *v;
    Vec3 *moving;
    Vec3 *aux1;
    Vec3 *aux2;
    float *radius;
    int *materialId;
    int *textureId;
    HittableType *type;

    __host__ __device__
    HitRecord *hit(int index, const Ray& ray, const Interval& ray_t) const {
        HittableType t = type[index];
        switch (t) {
            case Sphere:
                return hit_sphere(point[index], radius[index], ray, ray_t);
            case Triangle:
                return hit_triangle(point[index], u[index], v[index], ray, ray_t);
            case Quad:
                return hit_quad(point[index], u[index], v[index], aux1[index], ray, ray_t);
            default:
                return nullptr;
        }
        return nullptr;
    }
};

/*
    SoA style hittable list for GPU ray tracing

    - point: For spheres, this is the center. For triangles and quads, this is the first vertex.
    - u: For triangles and quads, this is the edge vector from the first vertex to the second vertex. For spheres, this is unused.
    - v: For triangles and quads, this is the edge vector from the first vertex to the third vertex. For spheres, this is unused.
    - moving: For moving spheres, this is the velocity vector. For static objects, this is unused.
    - aux1: For triangles, this is unused. For quads, this is (w = ) n / (n * n). For spheres, this is unused.
    - aux2: Currently unused, reserved for future use.
    - radius: For spheres, this is the radius. For triangles and quads, this is unused.
    - materialId: The ID of the material associated with this hittable object.
    - textureId: The ID of the texture associated with this hittable object (if any).

*/

__host__ __device__
HitRecord *hit_sphere(const Vec3 &center, float radius, const Ray &ray, const Interval& ray_t) {
    Vec3 oc = ray.origin - center;
    float a = ray.direction.dot(ray.direction);
    float b = 2.0f * oc.dot(ray.direction);
    float c = oc.dot(oc) - radius * radius;
    float discriminant = b * b - 4 * a * c;

    if (discriminant > 0) {
        float sqrt_disc = sqrtf(discriminant);
        float t1 = (-b - sqrt_disc) / (2.0f * a);
        if (t1 < ray_t.max && t1 > ray_t.min) {
            HitRecord *record = new HitRecord();
            record->t = t1;
            record->p = ray.at(t1);
            record->normal = (record->p - center) / radius;
            record->set_face_normal(ray, record->normal);
            return record;
        }
        float t2 = (-b + sqrt_disc) / (2.0f * a);
        if (t2 < ray_t.max && t2 > ray_t.min) {
            HitRecord *record = new HitRecord();
            record->t = t2;
            record->p = ray.at(t2);
            record->normal = (record->p - center) / radius;
            record->set_face_normal(ray, record->normal);
            return record;
        }
    }
    return nullptr;
}

__host__ __device__
HitRecord *hit_triangle(const Vec3& a, const Vec3& edge1, const Vec3& edge2, const Ray& ray, const Interval& ray_t) {
    Vec3 h = ray.direction.cross(edge2);
    float det = edge1.dot(h);
    if (fabsf(det) < 1e-6f) {
        return nullptr;
    }
    float f = 1.0f / det;
    Vec3 s = ray.origin - a;
    float u = f * s.dot(h);
    if (u < 0.0f || u > 1.0f) {
        return nullptr;
    }
    Vec3 q = s.cross(edge1);
    float v = f * ray.direction.dot(q);
    if (v < 0.0f || u + v > 1.0f) {
        return nullptr;
    }
    float t = f * edge2.dot(q);
    if (t < ray_t.min || t > ray_t.max) {
        return nullptr;
    }
    HitRecord *record = new HitRecord();
    record->t = t;
    record->p = ray.at(t);
    record->normal = edge1.cross(edge2).normalize();
    record->set_face_normal(ray, record->normal);
    record->u = u;
    record->v = v;
    return record;
}

__host__ __device__
HitRecord *hit_quad(const Vec3& q, const Vec3& u, const Vec3& v, const Vec3& w, const Ray& ray, const Interval& ray_t) {
    float denom = u.cross(v).dot(ray.direction);
    if (fabsf(denom) < 1e-6f) {
        return nullptr;
    }
    float t = (q - ray.origin).dot(u.cross(v)) / denom;
    if (t < ray_t.min || t > ray_t.max) {
        return nullptr;
    }
    Vec3 intersection = ray.at(t);
    Vec3 planar_hitpoint = intersection - q;
    float u_coord = planar_hitpoint.dot(u) / u.dot(u);
    float v_coord = planar_hitpoint.dot(v) / v.dot(v);
    if (u_coord < 0.0f || u_coord > 1.0f || v_coord < 0.0f || v_coord > 1.0f) {
        return nullptr;
    }
    HitRecord *record = new HitRecord();
    record->t = t;
    record->p = intersection;
    record->normal = u.cross(v).normalize();
    record->set_face_normal(ray, record->normal);
    record->u = u_coord;
    record->v = v_coord;
    return record;
}

#endif // CUDA_HITTABLE_CUH