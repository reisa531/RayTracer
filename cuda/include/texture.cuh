#ifndef CUDA_TEXTURE_CUH
#define CUDA_TEXTURE_CUH

#include "vec3.cuh"

enum TextureType {
    UnknownTexture = 0,
    SolidColor,
    Checker,
    Image,
    Perlin
};

struct TextureList {
    int count;
    Vec3 *color1;
    Vec3 *color2;
    Vec3 *color3;
    float *checkerInvScale;
    int *resourceId;
    // Perlin *perlin; // Placeholder for future Perlin noise parameters
    TextureType *type;

    __device__
    Vec3 sample(int index, float u, float v, const Point3& p) const {
        if (count <= 0 || index < 0 || index >= count ||
            color1 == nullptr || color2 == nullptr || color3 == nullptr ||
            checkerInvScale == nullptr || resourceId == nullptr || type == nullptr) {
            return Vec3(1.0f, 0.0f, 1.0f);
        }

        TextureType t = type[index];
        switch (t) {
            case SolidColor:
                return color1[index];
            case Checker: {
                float inv_scale = checkerInvScale[index];
                int ix = static_cast<int>(floorf(inv_scale * p.x));
                int iy = static_cast<int>(floorf(inv_scale * p.y));
                int iz = static_cast<int>(floorf(inv_scale * p.z));
                if ((ix + iy + iz) % 2 == 0) {
                    return color2[index];
                } else {
                    return color1[index];
                }
            }
            case Image:
                // Placeholder for future image texture sampling
                return Vec3(0.0f, 0.0f, 0.0f);
            case Perlin:
                // Placeholder for future Perlin noise sampling
                return Vec3(0.0f, 0.0f, 0.0f);
            default:
                return Vec3(1.0f, 1.0f, 1.0f); // Default to white for unknown types
        }
    }
};

#endif // CUDA_TEXTURE_CUH