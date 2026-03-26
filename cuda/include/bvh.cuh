#ifndef CUDA_BVH_CUH
#define CUDA_BVH_CUH

#include "aabb.cuh"
#include "hittable.cuh"

// flat SoA BVH representation for GPU ray tracing
struct BVH {
    int *left;
    int *right;
    AABB *bbox;
    int *hittableIndex; // Only valid for leaf nodes, nullptr for internal
    bool *isLeaf;
    HittableList *hittables;

    __host__ __device__
    HitRecord *hit(const Ray& ray, const Interval& ray_t, int node_index) const {
        if (!bbox[node_index].hit(ray, ray_t)) {
            return nullptr;
        }
        if (isLeaf[node_index]) {
            return hittables->hit(hittableIndex[node_index], ray, ray_t);
        }
        HitRecord *hit_left = hit(ray, ray_t, left[node_index]);
        HitRecord *hit_right = hit(ray, ray_t, right[node_index]);
        if (hit_left && hit_right) {
            return (hit_left->t < hit_right->t) ? hit_left : hit_right;
        }
        return hit_left ? hit_left : hit_right;
    }
};

#endif // CUDA_BVH_CUH