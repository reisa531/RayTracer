#ifndef CUDA_BVH_CUH
#define CUDA_BVH_CUH

#include "aabb.cuh"
#include "hittable.cuh"

// flat SoA BVH representation for GPU ray tracing
struct BVH {
    int *left;
    int *right;
    AABB *bbox;
    int *hittableIndex; // Only valid for leaf nodes, -1 for internal
    bool *isLeaf;
    HittableList *hittables;

    __host__ __device__
    bool hit(const Ray& ray, const Interval& ray_t, HitRecord& out_record, int node_index = 0) const {
        if (node_index < 0) {
            return false;
        }

        constexpr int STACK_CAPACITY = 2048;
        int stack[STACK_CAPACITY];
        int top = 0;
        stack[top++] = node_index;

        bool any_hit = false;
        float closest_t = ray_t.max;
        HitRecord temp_record;

        while (top > 0) {
            int current = stack[--top];
            if (current < 0) {
                continue;
            }

            if (!bbox[current].hit(ray, Interval(ray_t.min, closest_t))) {
                continue;
            }

            int obj_index = hittableIndex[current];
            if (obj_index >= 0) {
                if (hittables->hit(obj_index, ray, Interval(ray_t.min, closest_t), temp_record)) {
                    any_hit = true;
                    closest_t = temp_record.t;
                    out_record = temp_record;
                }
                continue;
            }

            if (top + 2 >= STACK_CAPACITY) {
                return any_hit;
            }

            stack[top++] = left[current];
            stack[top++] = right[current];
        }

        return any_hit;
    }
};

#endif // CUDA_BVH_CUH