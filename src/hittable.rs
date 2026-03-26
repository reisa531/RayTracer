use crate::utils;
use crate::vec3::Vec3;
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;
use crate::aabb::AABB;

use std::any::Any;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;
use::std::sync::Arc;

#[derive(Clone)]

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // NOTE: outward_normal is supposed to have unit length.

        self.front_face = *r.direction() * *outward_normal < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox
        }
    }

    pub fn object(&self) -> Arc<dyn Hittable> {
        self.object.clone()
    }

    pub fn offset(&self) -> Vec3 {
        self.offset
    }
}

impl Hittable for Translate {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(*r.origin() - self.offset, *r.direction(), r.time());

        if let Some(mut rec) = self.object.hit(&offset_r, ray_t) {
            rec.p += self.offset;
            return Some(rec);
        }

        None
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = utils::degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f64) * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = (j as f64) * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = (k as f64) * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox
        }
    }

    pub fn object(&self) -> Arc<dyn Hittable> {
        self.object.clone()
    }

    pub fn sin_theta(&self) -> f64 {
        self.sin_theta
    }

    pub fn cos_theta(&self) -> f64 {
        self.cos_theta
    }
}

impl Hittable for RotateY {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z()
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z()
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.object.hit(&rotated_r, ray_t) {
            rec.p = Point3::new(
                self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
                rec.p.y(),
                -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z()
            );

            rec.normal = Vec3::new(
                self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
                rec.normal.y(),
                -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z()
            );

            return Some(rec);
        }

        None
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub trait Hittable: Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABB;
}