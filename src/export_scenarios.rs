use crate::bvh::BVHNode;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::{cuboid, Quad};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::utils::{random_real, random_real_interval};
use crate::{Color, Point3, Vec3};

use rand::thread_rng;
use std::sync::Arc;

use crate::export_ir::export_to_ir_file;

pub fn build_main_default_scene_for_ir() -> BVHNode {
    let mut boxes1: HittableList = HittableList::default();
    let ground = Arc::new(Lambertian::new(0.48, 0.83, 0.53));

    let mut rng = thread_rng();

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_real_interval(&mut rng, 1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Box::new(cuboid(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world: HittableList = HittableList::default();

    let boxes1_bvh = HittableList::to_bvh(boxes1, &mut rng);
    world.push(Box::new(boxes1_bvh));

    let light = Arc::new(DiffuseLight::new(7.0, 7.0, 7.0));
    world.push(Box::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(0.7, 0.3, 0.1));
    world.push(Box::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(0.8, 0.8, 0.9, 1.0)),
    )));

    world.push(Box::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    let boundary1 = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(Box::new(ConstantMedium::new(
        boundary1,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    let boundary2 = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.push(Box::new(ConstantMedium::new(
        boundary2,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new(
        "./assets/earthmap.jpg",
    ))));
    world.push(Box::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.push(Box::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));

    let mut boxes2: HittableList = HittableList::default();
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let ns = 1000;
    for _ in 0..ns {
        let center = Point3::new(
            random_real_interval(&mut rng, 0.0, 165.0),
            random_real_interval(&mut rng, 0.0, 165.0),
            random_real_interval(&mut rng, 0.0, 165.0),
        );
        boxes2.push(Box::new(Sphere::new(center, 10.0, white.clone())));
    }

    let boxes2_bvh = Arc::new(HittableList::to_bvh(boxes2, &mut rng));
    world.push(Box::new(Translate::new(
        Arc::new(RotateY::new(boxes2_bvh, 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    HittableList::to_bvh(world, &mut rng)
}

fn build_bouncing_spheres_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();

    let checker_texture = CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    );
    let material_ground = Arc::new(Lambertian::from_texture(Arc::new(checker_texture)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_real(&mut rng);
            let center = Point3::new(
                (a as f64) + 0.9 * random_real(&mut rng),
                0.2,
                (b as f64) + 0.9 * random_real(&mut rng),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Color::new(
                        random_real(&mut rng),
                        random_real(&mut rng),
                        random_real(&mut rng),
                    );
                    let albedo = color.hadamard_product(color);
                    let material_sphere = Arc::new(Lambertian::from_color(albedo));
                    let center1 =
                        center + Vec3::new(0.0, random_real_interval(&mut rng, 0.0, 0.5), 0.0);
                    world.push(Box::new(Sphere::new_moving(
                        center,
                        center1,
                        0.2,
                        material_sphere,
                    )));
                } else if choose_mat < 0.95 {
                    let color = Color::new(
                        random_real_interval(&mut rng, 0.5, 1.0),
                        random_real_interval(&mut rng, 0.5, 1.0),
                        random_real_interval(&mut rng, 0.5, 1.0),
                    );
                    let fuzz = random_real_interval(&mut rng, 0.0, 0.5);
                    let material_sphere = Arc::new(Metal::from_color(color, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, material_sphere)));
                } else {
                    let material_sphere = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, material_sphere)));
                }
            }
        }
    }

    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(0.4, 0.2, 0.1)),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(0.7, 0.6, 0.5, 0.0)),
    )));

    HittableList::to_bvh(world, &mut rng)
}

fn build_checkered_spheres_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let checker_texture = CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    );
    let mat = Arc::new(Lambertian::from_texture(Arc::new(checker_texture)));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        mat.clone(),
    )));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat)));
    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_earth_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let earth_texture = Arc::new(ImageTexture::new("./assets/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));
    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_perlin_spheres_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));
    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_quads_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();

    world.push(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::new(1.0, 0.2, 0.2)),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::new(0.2, 1.0, 0.2)),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        Arc::new(Lambertian::new(0.2, 0.2, 1.0)),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        Arc::new(Lambertian::new(1.0, 0.5, 0.0)),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        Arc::new(Lambertian::new(0.2, 0.8, 0.8)),
    )));

    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_simple_light_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::from_texture(pertext.clone())),
    )));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::from_texture(pertext)),
    )));
    let difflight = Arc::new(DiffuseLight::new(4.0, 4.0, 4.0));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.push(Box::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_cornell_box_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let red = Arc::new(Lambertian::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::new(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::new(15.0, 15.0, 15.0));

    world.push(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    world.push(Box::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(cuboid(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    )));
    world.push(Box::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(cuboid(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    )));

    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

fn build_cornell_smoke_for_ir() -> BVHNode {
    let mut world: HittableList = HittableList::default();
    let red = Arc::new(Lambertian::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::new(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::new(7.0, 7.0, 7.0));

    world.push(Box::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.push(Box::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(cuboid(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 330.0, 165.0),
                white.clone(),
            )),
            15.0,
        )),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    let box2 = Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(cuboid(
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(165.0, 165.0, 165.0),
                white.clone(),
            )),
            -18.0,
        )),
        Vec3::new(130.0, 0.0, 65.0),
    ));

    world.push(Box::new(ConstantMedium::new(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    world.push(Box::new(ConstantMedium::new(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let mut rng = thread_rng();
    HittableList::to_bvh(world, &mut rng)
}

pub fn build_scene_for_ir(scenario: u32) -> Result<BVHNode, String> {
    match scenario {
        1 => Ok(build_bouncing_spheres_for_ir()),
        2 => Ok(build_checkered_spheres_for_ir()),
        3 => Ok(build_earth_for_ir()),
        4 => Ok(build_perlin_spheres_for_ir()),
        5 => Ok(build_quads_for_ir()),
        6 => Ok(build_simple_light_for_ir()),
        7 => Ok(build_cornell_box_for_ir()),
        8 => Ok(build_cornell_smoke_for_ir()),
        9 => Ok(build_main_default_scene_for_ir()),
        _ => Err(format!("unsupported scenario id: {}", scenario)),
    }
}

pub fn export_scene_ir(scenario: u32, path: &str) -> Result<(), String> {
    let scene = build_scene_for_ir(scenario)?;
    export_to_ir_file(&scene, path)
}

pub fn export_all_scene_ir(dir: &str) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    for scenario in 1..=9_u32 {
        let path = format!("{}/scenario_{}.ir", dir, scenario);
        export_scene_ir(scenario, &path)?;
        paths.push(path);
    }
    Ok(paths)
}
