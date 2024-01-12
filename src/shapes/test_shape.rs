use std::sync::{Arc, Mutex};

use glam::DVec3;

use crate::{ray::Ray, Object, intersection::Intersections};
use super::shape::Hittable;

#[derive(Clone, Debug)]
pub struct TestShape {
    saved_ray: Arc<Mutex<Option<Ray>>>
}

impl Hittable for TestShape {
    fn intersect<'a>(&self, ray: &Ray, _: &'a Object) -> Intersections<'a> {
        *self.saved_ray.lock().unwrap() = Some(*ray);
        Intersections::new()
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        point
    }
}

impl Default for TestShape {
    fn default() -> Self {
        Self {
            saved_ray: Arc::new(Mutex::new(None))
        }
    }
}

impl TestShape {
    pub fn new() -> Self {
        Self::default()
    }
}

impl PartialEq for TestShape {
    fn eq(&self, _other: &TestShape) -> bool {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use glam::{DVec3, dvec3};

    use crate::{Object, shapes::Shape};

    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn intersecting_a_saled_shape_with_a_ray() {
        let r = Ray::new(
            DVec3::new(0.0, 0.0, -5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = TestShape::default();
        let o = Object::new(Shape::TestShape(s.clone()))
            .with_scale(2.0, 2.0, 2.0)
            .transform();
        o.intersect(&r);
        assert_eq!(s.saved_ray.lock().unwrap().unwrap().origin, dvec3(0.0, 0.0, -2.5));
        assert_eq!(s.saved_ray.lock().unwrap().unwrap().direction, dvec3(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(
            DVec3::new(0.0, 0.0, -5.0), 
            DVec3::new(0.0, 0.0, 1.0)
        );
        let s = TestShape::default();
        let o = Object::new(Shape::TestShape(s.clone()))
            .with_translation(5.0, 0.0, 0.0)
            .transform();
        o.intersect(&r);
        assert_eq!(s.saved_ray.lock().unwrap().unwrap().origin, dvec3(-5.0, 0.0, -5.0));
        assert_eq!(s.saved_ray.lock().unwrap().unwrap().direction, dvec3(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = TestShape::default();
        let o = Object::new(Shape::TestShape(s.clone()))
            .with_translation(0.0, 1.0, 0.0)
            .transform();
        let n = o.normal_at(dvec3(0.0, 1.70711, -0.70711));
        assert!(n.abs_diff_eq(dvec3(0.0, 0.70711, -0.70711), EPSILON));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = TestShape::default();
        let o = Object::new(Shape::TestShape(s.clone()))
            .with_rotation_z(PI / 5.0)
            .with_scale(1.0, 0.5, 1.0)
            .transform();
        let n = o.normal_at(dvec3(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        assert!(n.abs_diff_eq(dvec3(0.0, 0.97014, -0.24254), EPSILON));
    }
}