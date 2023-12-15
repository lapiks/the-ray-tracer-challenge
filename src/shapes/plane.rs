use std::f64::EPSILON;

use glam::DVec3;

use crate::ray::Ray;
use super::shape::Hittable;

/// infinite xz plane 
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {}

impl Hittable for Plane {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        if ray.direction.y.abs() < EPSILON {
            return vec![]
        }
        vec![
            -ray.origin.y / ray.direction.y
        ]
    }

    fn normal_at(&self, _: DVec3) -> DVec3 {
        DVec3::new(0.0, 1.0, 0.0)
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
    use glam::dvec3;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        assert_eq!(p.normal_at(dvec3(0.0, 0.0, 0.0)), dvec3(0.0, 1.0, 0.0));
        assert_eq!(p.normal_at(dvec3(10.0, 0.0, -10.0)), dvec3(0.0, 1.0, 0.0));
        assert_eq!(p.normal_at(dvec3(-5.0, 0.0, 150.0)), dvec3(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = Ray {
            origin: dvec3(0.0, 10.0, 0.0),
            direction: dvec3(0.0, 0.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_ray_coplanar_to_the_plane() {
        let p = Plane::default();
        let r = Ray {
            origin: dvec3(0.0, 0.0, 0.0),
            direction: dvec3(0.0, 0.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray {
            origin: dvec3(0.0, 1.0, 0.0),
            direction: dvec3(0.0, -1.0, 0.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray {
            origin: dvec3(0.0, -1.0, 0.0),
            direction: dvec3(0.0, 1.0, 1.0)
        };
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}