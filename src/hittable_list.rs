pub use std::vec;
use crate::hittable::Hittable;
pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::hittable::HitRecord;
use crate::interval::Interval;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;

        for object in self {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}