use crate::vec3::Point3;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere {
            center,
            radius: radius.max(0.0)
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = (*r.direction()).length_squared();
        let h = *r.direction() * oc;
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return None;
            }
        }

        let recordp = r.at(root);
        let mut record = HitRecord {
            t: root,
            normal: (recordp - self.center) / self.radius,
            p: recordp,
            front_face: true
        };

        let outward_normal = (record.p.clone() - self.center) / self.radius;
        record.set_face_normal(r, &outward_normal);

        Some(record)
    }
}