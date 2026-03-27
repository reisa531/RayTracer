#ifndef CUDA_PI_CUH
#define CUDA_PI_CUH

// Define M_PI for CUDA device and host code if not provided by the platform.
// Use a float literal since most usages in the project operate in float.
#ifndef M_PI
#define M_PI 3.14159265358979323846f
#endif

#endif // CUDA_PI_CUH
