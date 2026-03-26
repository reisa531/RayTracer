pub use crate::hittable::Hittable;
pub use crate::aabb::AABB;
pub use crate::interval::Interval;
pub use crate::utils::random_integer_interval;

pub use std::vec;
pub use std::sync::Arc;
pub use std::cmp::Ordering;
pub use rand::RngCore;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB
}

impl BVHNode {
    pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
        let box_a = a.bounding_box();
        let box_b = b.bounding_box();
        let a_axis_interval = box_a.axis_interval(axis_index);
        let b_axis_interval = box_b.axis_interval(axis_index);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }

    pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl BVHNode {
    pub fn build(objects: &mut [Arc<dyn Hittable>], rng: &mut dyn RngCore) -> Self {
        let mut bbox = AABB::EMPTY;
        for object in &mut *objects {
            bbox = AABB::from_aabbs(&bbox, &object.bounding_box());
        }

        let axis: i32 = bbox.longest_axis();

        let comparator = if axis == 0 { Self::box_x_compare }
            else if axis == 1 { Self::box_y_compare }
            else { Self::box_z_compare };

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        let object_span: usize = objects.len();
        if object_span == 1 {
            left = Arc::clone(&objects[0]);
            right = Arc::clone(&objects[0]);
        }
        else if object_span == 2 {
            left = Arc::clone(&objects[0]);
            right = Arc::clone(&objects[1]);
        }
        else {
            objects.sort_unstable_by(comparator);

            let mid = object_span / 2;
            let (left_objects, right_objects) = objects.split_at_mut(mid);
            left = Arc::new(BVHNode::build(left_objects, rng));
            right = Arc::new(BVHNode::build(right_objects, rng));
        }

        let bbox = AABB::from_aabbs(&left.bounding_box(), &right.bounding_box());

        Self {
            left,
            right,
            bbox
        }
    }

    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, rng: &mut dyn RngCore) -> Self {
        Self::build(&mut objects, rng)
    }

    pub fn left(&self) -> Arc<dyn Hittable> {
        self.left.clone()
    }

    pub fn right(&self) -> Arc<dyn Hittable> {
        self.right.clone()
    }

    pub fn bbox_ref(&self) -> &AABB {
        &self.bbox
    }
}

impl Hittable for BVHNode {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn hit(&self, r: &crate::Ray, ray_t: crate::Interval) -> Option<crate::HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_left = self.left.hit(r, ray_t);
        let hit_right = self.right.hit(r,
            Interval::new(
                ray_t.min,
                hit_left.as_ref().map(|rec| rec.t).unwrap_or(ray_t.max)
            )
        );

        if hit_right.is_some() {
            return hit_right;
        }
        else {
            return hit_left;
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}