use raytracer::{AABB, Interval, Point3, Ray, Vec3};

#[test]
fn from_points_sorts_axis_bounds() {
    let bbox = AABB::from_points(Point3::new(3.0, -1.0, 5.0), Point3::new(-2.0, 4.0, 1.0));

    assert_eq!(bbox.x, Interval::new(-2.0, 3.0));
    assert_eq!(bbox.y, Interval::new(-1.0, 4.0));
    assert_eq!(bbox.z, Interval::new(1.0, 5.0));
}

#[test]
fn hit_returns_true_for_intersecting_ray() {
    let bbox = AABB::from_points(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 0.0);

    assert!(bbox.hit(&ray, Interval::new(0.0, f64::INFINITY)));
}

#[test]
fn hit_accumulates_interval_across_all_axes() {
    let bbox = AABB::from_points(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.05, 0.05), 0.0);

    assert!(!bbox.hit(&ray, Interval::new(0.0, f64::INFINITY)));
}

#[test]
fn hit_respects_ray_t_window() {
    let bbox = AABB::from_points(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0));
    let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0), 0.0);

    assert!(!bbox.hit(&ray, Interval::new(0.0, 0.5)));
}
