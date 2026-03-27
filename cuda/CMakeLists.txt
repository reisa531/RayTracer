cmake_minimum_required(VERSION 3.10)
set(CMAKE_EXPORT_COMPILE_COMMANDS OFF)

if(POLICY CMP0146)
	cmake_policy(SET CMP0146 OLD)
endif()

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

enable_testing()

add_executable(cuda_parser_smoke tests/cuda_parser_smoke.cu)
target_include_directories(cuda_parser_smoke PRIVATE
	${CUDA_INCLUDE_DIRS}
	${CMAKE_CURRENT_SOURCE_DIR}/include
)

add_executable(gpu_render gpu_render.cu)
target_include_directories(gpu_render PRIVATE
	${CUDA_INCLUDE_DIRS}
	${CMAKE_CURRENT_SOURCE_DIR}/include
)

add_test(
	NAME cuda_parser_smoke_test
	COMMAND cuda_parser_smoke ${CMAKE_CURRENT_SOURCE_DIR}/../ir/scenario_9.ir
)