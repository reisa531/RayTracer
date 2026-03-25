cmake_minimum_required(VERSION 3.10)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(RayTracerCUDA LANGUAGES CXX CUDA)

set(CMAKE_CUDA_STANDARD 17)
set(CMAKE_CUDA_STANDARD_REQUIRED ON)

set(CMAKE_CUDA_ARCHITECTURES 75 80 86)
find_program(NVCC_EXECUTABLE nvcc PATHS /usr/local/cuda/bin /usr/bin ENV PATH)
if(NVCC_EXECUTABLE)
	set(CMAKE_CUDA_COMPILER "${NVCC_EXECUTABLE}" CACHE PATH "CUDA compiler" FORCE)
endif()

find_package(CUDA REQUIRED)

include_directories(${CUDA_INCLUDE_DIRS})
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/cuda)
include_directories(include)

set(CUDA_NVCC_FLAGS ${CUDA_NVCC_FLAGS};-O3)