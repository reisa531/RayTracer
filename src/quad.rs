use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
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

    pub fn q(&self) -> Point3 {
        self.q
    }

    pub fn u(&self) -> Vec3 {
        self.u
    }

    pub fn v(&self) -> Vec3 {
        self.v
    }

    pub fn w(&self) -> Vec3 {
        self.w
    }

    pub fn material(&self) -> Arc<dyn Material> {
        self.mat.clone()
    }
}

impl Hittable for Quad {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

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

pub fn cuboid(a: Point3, b: Point3, mat: Arc<dyn Material>) -> HittableList {
    let mut sides = HittableList::default();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.push(Box::new(Quad::new(min, dx, dy, mat.clone())));
    sides.push(Box::new(Quad::new(min, dx, dz, mat.clone())));
    sides.push(Box::new(Quad::new(min, dy, dz, mat.clone())));
    sides.push(Box::new(Quad::new(max, -dx, -dy, mat.clone())));
    sides.push(Box::new(Quad::new(max, -dx, -dz, mat.clone())));
    sides.push(Box::new(Quad::new(max, -dy, -dz, mat.clone())));

    sides
}