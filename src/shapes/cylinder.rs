use std::{mem::swap, f64::EPSILON};

use glam::{DVec3, dvec3};

use crate::{ray::Ray, Object, intersection::{Intersections, Intersection}, bounds::BoundingBox};
use super::shape::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cylinder {
    min: f64,
    max: f64,
    closed: bool,
}

impl Hittable for Cylinder {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;
        if f64::abs(a) < f64::EPSILON {
            return self.intersect_caps(ray, object);
        }

        let b = 2.0 * ray.origin.x * ray.direction.x +
            2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.0;

        let disc = b*b - 4.0*a*c;
        if disc < 0.0 {
            return Intersections::new();
        }

        let mut t0 = (-b - f64::sqrt(disc)) / (2.0 * a);
        let mut t1 = (-b + f64::sqrt(disc)) / (2.0 * a);

        if t0 > t1 {
            swap(&mut t0, &mut t1);
        }

        let y0 = ray.origin.y + t0 * ray.direction.y;
        let mut xs = Vec::default();
        if self.min < y0 && y0 <self.max {
            xs.push(Intersection::new(t0, object));
        }

        let y1 = ray.origin.y + t1 * ray.direction.y;
        if self.min < y1 && y1 <self.max {
            xs.push(Intersection::new(t1, object));
        }

        let mut intersections = Intersections::new()
        .with_intersections(xs);

        intersections.append(self.intersect_caps(ray, object));
        intersections
    }

    fn normal_at(&self, point: DVec3, _: f64, _: f64) -> DVec3 {
        let dist = point.x * point.x + point.z * point.z;
        if dist < 1.0 && point.y >= self.max - EPSILON {
            dvec3(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= self.min + EPSILON {
            dvec3(0.0, -1.0, 0.0)
        } else {
            dvec3(point.x, 0.0, point.z)
        }
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::default()
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            closed: true,
        }
    }
}

impl Cylinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_min(mut self, min: f64) -> Self {
        self.min = min;
        self
    }

    pub fn with_max(mut self, max: f64) -> Self {
        self.max = max;
        self
    }

    pub fn with_closed(mut self, closed: bool) -> Self {
        self.closed = closed;
        self
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;

        (x * x + z * z) <= 1.0
    }

    fn intersect_caps<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        if !self.closed || f64::abs(ray.direction.y) < f64::EPSILON {
            return Intersections::new();
        }

        let mut xs = Vec::default();

        let t = (self.min - ray.origin.y) / ray.direction.y;
        if Cylinder::check_cap(ray, t) {
            xs.push(
                Intersection::new(t, object)
            );
        }

        let t = (self.max - ray.origin.y) / ray.direction.y;
        if Cylinder::check_cap(ray, t) {
            xs.push(
                Intersection::new(t, object)
            );
        }

        Intersections::new()
        .with_intersections(xs)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::Shape;

    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Object::new(Shape::Cylinder(Cylinder::new()));
        {
            let r = Ray::new(dvec3(1.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0));
            assert_eq!(cyl.intersect(&r).count(), 0);
        }
        {
            let r = Ray::new(dvec3(0.0, 0.0, 0.0), dvec3(0.0, 1.0, 0.0));
            assert_eq!(cyl.intersect(&r).count(), 0);
        }
        {
            let r = Ray::new(dvec3(0.0, 0.0, -5.0), dvec3(1.0, 1.0, 1.0).normalize());
            assert_eq!(cyl.intersect(&r).count(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Object::new(Shape::Cylinder(Cylinder::new()));
        {
            let r = Ray::new(dvec3(1.0, 0.0, -5.0), dvec3(0.0, 0.0, 1.0));
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
            assert_eq!(xs[0].t(), 5.0);
            assert_eq!(xs[1].t(), 5.0);
        }
        {
            let r = Ray::new(dvec3(0.0, 0.0, -5.0), dvec3(0.0, 0.0, 1.0));
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
            assert_eq!(xs[0].t(), 4.0);
            assert_eq!(xs[1].t(), 6.0);
        }
        {
            let r = Ray::new(dvec3(0.5, 0.0, -5.0), dvec3(0.1, 1.0, 1.0).normalize());
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
            assert!(f64::abs(xs[0].t() - 6.80798) < EPSILON);
            assert!(f64::abs(xs[1].t() - 7.08872) < EPSILON);
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Object::new(Shape::Cylinder(Cylinder::new()));
        assert_eq!(cyl.normal_at(dvec3(1.0, 0.0, 0.0), 0.0, 0.0), dvec3(1.0, 0.0, 0.0));
        assert_eq!(cyl.normal_at(dvec3(0.0, 5.0, -1.0), 0.0, 0.0), dvec3(0.0, 0.0, -1.0));
        assert_eq!(cyl.normal_at(dvec3(0.0, -2.0, 1.0), 0.0, 0.0), dvec3(0.0, 0.0, 1.0));
        assert_eq!(cyl.normal_at(dvec3(-1.0, 1.0, 0.0), 0.0, 0.0), dvec3(-1.0, 0.0, 0.0));
    }

    #[test]
    fn the_default_min_and_max_for_a_cylinder() {
        let cyl = Cylinder::new();
        assert_eq!(cyl.min, f64::NEG_INFINITY);
        assert_eq!(cyl.max, f64::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = Object::new(
            Shape::Cylinder(
                Cylinder::new()
                .with_min(1.0)
                .with_max(2.0)
                .with_closed(false)
            )
        );
        {
            let r = Ray::new(
                dvec3(0.0, 1.5, 0.0), 
                dvec3(0.1, 1.0, 0.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 3.0, -5.0), 
                dvec3(0.0, 0.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 0.0, -5.0), 
                dvec3(0.0, 0.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 2.0, -5.0), 
                dvec3(0.0, 0.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 1.0, -5.0), 
                dvec3(0.0, 0.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 0);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 1.5, -2.0), 
                dvec3(0.0, 0.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::default();
        assert_eq!(cyl.closed, true);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let cyl = Object::new(
            Shape::Cylinder(
                Cylinder::new()
                .with_min(1.0)
                .with_max(2.0)
                .with_closed(true)
            )
        );
        {
            let r = Ray::new(
                dvec3(0.0, 3.0, 0.0), 
                dvec3(0.0, -1.0, 0.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 3.0, -2.0), 
                dvec3(0.0, -1.0, 2.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 4.0, -2.0), 
                dvec3(0.0, -1.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
        {
            let r = Ray::new(
                dvec3(0.0, 0.0, -2.0), 
                dvec3(0.0, 1.0, 2.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
        {
            let r = Ray::new(
                dvec3(0.0, -1.0, -2.0), 
                dvec3(0.0, 1.0, 1.0)
            );
            let xs = cyl.intersect(&r);
            assert_eq!(xs.count(), 2);
        }
    }
}