#ifndef CUDA_MANAGER_CUH
#define CUDA_MANAGER_CUH

#include "material.cuh"
#include "bvh.cuh"
#include "texture.cuh"

struct Manager {
    BVH *bvh;
    MaterialList *materials;
    TextureList *textures;
};

#endif // CUDA_MANAGER_CUH