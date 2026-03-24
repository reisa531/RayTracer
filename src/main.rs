use raytracer::CheckerTexture;
use raytracer::Quad;
use raytracer::Vec3;
use raytracer::Color;
use raytracer::Dielectric;
use raytracer::Point3;
use raytracer::Sphere;
use raytracer::HittableList;
use raytracer::Camera;
use raytracer::Lambertian;
use raytracer::Metal;
use raytracer::material::DiffuseLight;
use raytracer::quad::cuboid;
use raytracer::texture::ImageTexture;
use raytracer::texture::NoiseTexture;
use raytracer::hittable::RotateY;
use raytracer::hittable::Translate;
use raytracer::utils::random_real;
use raytracer::utils::random_real_interval;

use std::sync::Arc;

fn bouncing_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker_texture = CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9)
    );
    let material_ground = Arc::new(Lambertian::from_texture(Arc::new(checker_texture)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, -1.0), 1000.0, material_ground)));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_real(&mut rng);
            let center = Point3::new(
                (a as f64) + 0.9 * random_real(&mut rng),
                0.2,
                (b as f64) + 0.9 * random_real(&mut rng)
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let color = Color::new(random_real(&mut rng), random_real(&mut rng), random_real(&mut rng));
                    let albedo = color.hadamard_product(color);
                    let material_sphere = Arc::new(Lambertian::from_color(albedo));
                    let center1 = center + Vec3::new(0.0, random_real_interval(&mut rng, 0.0, 0.5), 0.0);
                    world.push(Box::new(Sphere::new_moving(center, center1, 0.2, material_sphere)));
                }
                else if choose_mat < 0.95 {
                    let color = Color::new(
                        random_real_interval(&mut rng, 0.5, 1.0),
                        random_real_interval(&mut rng, 0.5, 1.0),
                        random_real_interval(&mut rng, 0.5, 1.0)
                    );
                    let fuzz = random_real_interval(&mut rng, 0.0, 0.5);
                    let material_sphere = Arc::new(Metal::from_color(color, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, material_sphere)));
                }
                else {
                    let material_sphere = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, material_sphere)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(Point3::new(0.0,1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(0.4, 0.2, 0.1));
    world.push(Box::new(Sphere::new(Point3::new(-4.0,1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(0.7, 0.6, 0.5, 0.0));
    world.push(Box::new(Sphere::new(Point3::new(4.0,1.0, 0.0), 1.0, material3)));

    let cam = Camera::new(16.0 / 9.0, 400,
            100, 50, 20.0,
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.6, 10.0,
            Color::new(0.70, 0.80, 1.00));
    
    let world = HittableList::to_bvh(world, &mut rng);
    
    cam.render(&world);
}

fn checkered_spheres() {
    let mut world: HittableList = HittableList::default();

    let checker_texture = CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9)
    );
    let mat = Arc::new(Lambertian::from_texture(Arc::new(checker_texture)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, mat.clone())));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, mat)));

    let mut rng = rand::thread_rng();

    let cam = Camera::new(16.0 / 9.0, 400,
            100, 50, 20.0,
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.6, 10.0,
            Color::new(0.70, 0.80, 1.00));
    
    let world = HittableList::to_bvh(world, &mut rng);
    
    cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("./assets/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);
    
    let cam = Camera::new(16.0 / 9.0, 1200,
            500, 50, 20.0,
            Point3::new(0.0, 0.0, -12.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0, 10.0,
            Color::new(0.70, 0.80, 1.00));

    let mut world = HittableList::default();
    world.push(Box::new(globe));
    
    cam.render(&world);
}

fn perlin_spheres() {
    let mut world: HittableList = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(pertext)))));

    let cam = Camera::new(16.0 / 9.0, 400,
            100, 50, 20.0,
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0, 10.0,
            Color::new(0.70, 0.80, 1.00));

    cam.render(&world);
}

fn quads() {
    let mut world: HittableList = HittableList::default();

    let left_red = Arc::new(Lambertian::new(1.0, 0.2, 0.2));
    let back_green = Arc::new(Lambertian::new(0.2, 1.0, 0.2));
    let right_blue = Arc::new(Lambertian::new(0.2, 0.2, 1.0));
    let upper_orange = Arc::new(Lambertian::new(1.0, 0.5, 0.0));
    let lower_teal = Arc::new(Lambertian::new(0.2, 0.8, 0.8));

    world.push(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red
    )));

    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green
    )));

    world.push(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue
    )));

    world.push(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange
    )));

    world.push(Box::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal
    )));

    let cam = Camera::new(1.0, 400, 100,
            50, 80.0,
            Point3::new(0.0, 0.0, 9.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0, 9.0,
            Color::new(0.70, 0.80, 1.00));

    let mut rng = rand::thread_rng();
    let world = HittableList::to_bvh(world, &mut rng);

    cam.render(&world);
}

fn simple_light() {
    let mut world: HittableList = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(pertext)))));

    let difflight = Arc::new(DiffuseLight::new(4.0, 4.0, 4.0));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    world.push(Box::new(Quad::new(
        Vec3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight)
    ));

    let cam = Camera::new(16.0 / 9.0, 400,
            100, 50, 20.0,
            Point3::new(26.0, 3.0, 6.0),
            Point3::new(0.0, 2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0, 10.0,
            Color::new(0.0, 0.0, 0.0));

    cam.render(&world);
}

fn cornell_box() {
    let mut world: HittableList = HittableList::default();

    let red   = Arc::new(Lambertian::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::new(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::new(15.0, 15.0, 15.0));

    world.push(Box::new(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green.clone())));
    world.push(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red.clone())));
    world.push(Box::new(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone())));
    world.push(Box::new(Quad::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.push(Box::new(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.push(Box::new(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Box::new(Translate::new(Arc::new(RotateY::new(Arc::new(cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone()
    )), 15.0)), Vec3::new(265.0, 0.0, 295.0)));

    let box2 = Box::new(Translate::new(Arc::new(RotateY::new(Arc::new(cuboid(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone()
    )), -18.0)), Vec3::new(130.0, 0.0, 65.0)));

    world.push(box1);
    world.push(box2);

    let cam = Camera::new(1.0, 600,
            200, 50, 40.0,
            Point3::new(278.0, 278.0, -800.0),
            Point3::new(278.0, 278.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.0, 10.0,
            Color::new(0.0, 0.0, 0.0));

    cam.render(&world);
}

fn main() {
    let scenario = 7;

    match scenario {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        _ => eprintln!("Invalid!")
    }
}