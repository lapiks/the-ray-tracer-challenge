use std::f64::EPSILON;

use glam::{DVec3, dvec3};

use crate::{ray::Ray, intersection::{Intersections, Intersection}, Object, bounds::BoundingBox};
use super::shape::Hittable;

/// infinite xz plane 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Hittable for Plane {
    fn intersect<'a>(&self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if ray.direction.y.abs() > EPSILON {
            xs.push(
                Intersection::new(
                    -ray.origin.y / ray.direction.y,
                    object
                )
            );
        }

        xs        
    }

    fn normal_at(&self, _: DVec3, _: f64, _: f64) -> DVec3 {
        DVec3::new(0.0, 1.0, 0.0)
    }

    fn bounds(&self) -> BoundingBox {
        BoundingBox::new(
            dvec3(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            dvec3(f64::INFINITY, 0.0, f64::INFINITY),
        )
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {}
    }
}

impl Plane {
    pub fn new() -> Self {
        Self::default()
    }
}


#[cfg(test)]
mod tests {
    use std::f64::{NEG_INFINITY, INFINITY};

    use glam::dvec3;

    use crate::shapes::Shape;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        assert_eq!(p.normal_at(dvec3(0.0, 0.0, 0.0), 0.0, 0.0), dvec3(0.0, 1.0, 0.0));
        assert_eq!(p.normal_at(dvec3(10.0, 0.0, -10.0), 0.0, 0.0), dvec3(0.0, 1.0, 0.0));
        assert_eq!(p.normal_at(dvec3(-5.0, 0.0, 150.0), 0.0, 0.0), dvec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Object::new(Shape::Plane(Plane::default()));
        let r = Ray {
            origin: dvec3(0.0, 10.0, 0.0),
            direction: dvec3(0.0, 0.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn intersect_with_a_ray_coplanar_to_the_plane() {
        let p = Object::new(Shape::Plane(Plane::default()));
        let r = Ray {
            origin: dvec3(0.0, 0.0, 0.0),
            direction: dvec3(0.0, 0.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Object::new(Shape::Plane(Plane::default()));
        let r = Ray {
            origin: dvec3(0.0, 1.0, 0.0),
            direction: dvec3(0.0, -1.0, 0.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t(), 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Object::new(Shape::Plane(Plane::default()));
        let r = Ray {
            origin: dvec3(0.0, -1.0, 0.0),
            direction: dvec3(0.0, 1.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.count(), 1);
        assert_eq!(xs[0].t(), 1.0);
    }

    #[test]
    fn a_plane_has_a_bounding_box() {
        let p = Plane::default();
        let bb = p.bounds();
        assert_eq!(bb.min(), dvec3(NEG_INFINITY, 0.0, NEG_INFINITY));
        assert_eq!(bb.max(), dvec3(INFINITY, 0.0, INFINITY));
    }
}