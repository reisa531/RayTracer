use std::f64::INFINITY;
pub use std::sync::Arc;

pub use crate::color::Color;
pub use crate::hittable::Hittable;
pub use crate::vec3::Vec3;
pub use crate::material::Isotropic;
pub use crate::material::Material;
pub use crate::texture::Texture;
pub use crate::aabb::AABB;
pub use crate::ray::Ray;
pub use crate::interval::Interval;
pub use crate::hittable::HitRecord;
use crate::utils::random_real;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo))
        }
    }

    pub fn from_texture(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(tex))
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut rec1= self.boundary.hit(r, Interval::UNIVERSE)?;
        let mut rec2 = self.boundary.hit(r, Interval::new(rec1.t + 0.0001, INFINITY))?;

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let mut rng = rand::thread_rng();

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_real(&mut rng).ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        let normal = Vec3::new(1.0, 0.0, 0.0);
        let front_face = true;
        let mat = self.phase_function.clone();

        Some(HitRecord{
            p,
            normal,
            mat,
            u: 0.0,
            v: 0.0,
            t,
            front_face
        })
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box().clone()
    }
}