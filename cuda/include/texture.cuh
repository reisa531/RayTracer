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
    // int *resourceId; // Placeholder for future GPU texture resources
    // Perlin *perlin; // Placeholder for future Perlin noise parameters
    TextureType *type;

    __device__
    Vec3 sample(int index, float u, float v, const Point3& p) const {
        TextureType t = type[index];
        switch (t) {
            case SolidColor:
                return color1[index];
            case Checker: {
                int checker_u = static_cast<int>(floorf(u * 10.0f));
                int checker_v = static_cast<int>(floorf(v * 10.0f));
                if ((checker_u + checker_v) % 2 == 0) {
                    return color1[index];
                } else {
                    return color2[index];
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