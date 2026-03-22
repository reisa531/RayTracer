use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;

use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    d: f64
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = Vec3::cross(&u, &v);
        let normal = n.unit();
        let d = normal * q;
        let w = n / (n * n);

        Self {
            q,
            u,
            v,
            w,
            normal,
            mat,
            bbox: AABB::from_quad(q, u, v),
            d
        }
    }

    fn is_interior(a: f64, b: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let denom = self.normal * *r.direction();
        
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal * *r.origin()) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);
        let planar_hitpoint = intersection - self.q;
        let alpha = self.w * Vec3::cross(&planar_hitpoint, &self.v);
        let beta = self.w * Vec3::cross(&self.u, &planar_hitpoint);

        if !Self::is_interior(alpha, beta) {
            return None;
        }

        let mut record  = HitRecord {
            t,
            normal: self.normal,
            mat: self.mat.clone(),
            p: intersection,
            front_face: true,
            u: alpha,
            v: beta
        };

        record.set_face_normal(r, &self.normal);

        Some(record)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}