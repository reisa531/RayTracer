pub use std::vec;
pub use std::sync::Arc;
pub use rand::RngCore;

pub use crate::hittable::Hittable;
pub use crate::vec3::Vec3;
pub use crate::ray::Ray;
pub use crate::hittable::HitRecord;
pub use crate::interval::Interval;
pub use crate::aabb::AABB;
pub use crate::bvh::BVHNode;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: AABB
}

impl HittableList {
    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::from_aabbs(&self.bbox, &(object.bounding_box()));
        self.objects.push(object);
    }

    pub fn to_bvh(list: HittableList, rng: &mut dyn RngCore) -> BVHNode {
        let objects_arc: Vec<Arc<dyn Hittable>> = list.objects.into_iter().map(|b| Arc::from(b)).collect();
        let bvh_root = BVHNode::new(objects_arc, rng);
        bvh_root
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = None;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }

    fn bounding_box(&self) -> crate::AABB {
        self.bbox.clone()
    }
}