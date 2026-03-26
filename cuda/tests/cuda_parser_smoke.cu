#include "../include/parser.cuh"

#include <iostream>
#include <string>

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "usage: cuda_parser_smoke <ir_path>" << std::endl;
        return 2;
    }

    ParsedIR ir;
    std::string err;
    if (!parse_ir_file(argv[1], ir, &err)) {
        std::cerr << "parse_ir_file failed: " << err << std::endl;
        return 1;
    }

    if (ir.hittables.empty() || ir.bvh.empty() || ir.materials.empty()) {
        std::cerr << "parsed IR has empty critical sections" << std::endl;
        return 1;
    }

    std::cout << "textures=" << ir.textures.size()
              << " materials=" << ir.materials.size()
              << " hittables=" << ir.hittables.size()
              << " bvh=" << ir.bvh.size()
              << " resources=" << ir.resources.size()
              << std::endl;

    return 0;
}
