use raytracer::export_ir::parse_ir_text;
use raytracer::export_scenarios::{export_all_scene_ir, export_scene_ir};

use std::fs;
use std::path::Path;
use std::process::Command;

fn generate_ir(path: &str) {
    export_scene_ir(9, path).expect("failed to export IR");
}

#[test]
fn test_export_and_parse_ir() {
    let ir_path = "ir/scenario_9.ir";
    generate_ir(ir_path);

    let content = fs::read_to_string(ir_path).expect("failed to read exported IR");
    let parsed = parse_ir_text(&content).expect("failed to parse exported IR");

    assert!(parsed.texture_count > 0, "texture section should not be empty");
    assert!(parsed.material_count > 0, "material section should not be empty");
    assert!(parsed.hittable_count > 0, "hittable section should not be empty");
    assert!(parsed.bvh_count > 0, "bvh section should not be empty");
}

#[test]
fn test_export_all_main_scenes() {
    let paths = export_all_scene_ir("ir").expect("failed to export all main scene IR files");
    assert_eq!(paths.len(), 9);

    for idx in 1..=9 {
        let path = format!("ir/scenario_{}.ir", idx);
        assert!(Path::new(&path).exists(), "missing exported file: {}", path);
    }
}

#[test]
fn test_cuda_parser_smoke() {
    let ir_path = "ir/scenario_9.ir";
    generate_ir(ir_path);

    let nvcc_check = Command::new("nvcc").arg("--version").output();
    if nvcc_check.is_err() {
        eprintln!("skip cuda parser smoke test: nvcc not found");
        return;
    }

    let binary_path = "target/cuda_parser_smoke";
    let source_path = "cuda/tests/cuda_parser_smoke.cu";

    let compile_status = Command::new("nvcc")
        .args([
            "-std=c++17",
            "-Icuda/include",
            source_path,
            "-o",
            binary_path,
        ])
        .status()
        .expect("failed to spawn nvcc");

    assert!(compile_status.success(), "nvcc failed to compile cuda parser smoke program");
    assert!(Path::new(binary_path).exists(), "compiled CUDA test binary missing");

    let run_status = Command::new(binary_path)
        .arg(ir_path)
        .status()
        .expect("failed to run cuda parser smoke binary");

    assert!(run_status.success(), "cuda parser smoke binary failed");
}
