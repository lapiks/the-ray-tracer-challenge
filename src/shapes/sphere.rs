use glam::Vec3;

use crate::ray::Ray;
use super::shape::Hittable;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        // -1.0 for radius*radius with radius = 1
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b*b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Vec::default();
        } 

        let sqrt_disc = discriminant.sqrt();
        let inv_denom = 1.0 / (2.0 * a);

        vec![
            (-b - sqrt_disc) * inv_denom,
            (-b + sqrt_disc) * inv_denom
        ]
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        // for a unit sphere at 0,0,0
        point.normalize()
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {}
    }
}

impl Sphere {
    pub fn new() -> Self {
        Self::default()
    }
}


#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use glam::Vec3;

    use crate::{object::Object, shapes::shape::Shape};

    use super::*;

    const EPSILON: f32 = 0.00001;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, -5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            Vec3::new(0.0, 1.0, -5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(
            Vec3::new(0.0, 2.0, -5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
    
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, 0.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, 5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, -5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s))
            .with_scale(2.0, 2.0, 2.0);
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0).unwrap().t(), 3.0);
        assert_eq!(xs.get(1).unwrap().t(), 7.0);
    } 

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, -5.0), 
            Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let o = Object::new(Shape::Sphere(s))
            .with_translation(5.0, 0.0, 0.0);
        let xs = o.intersect(&r);
        assert_eq!(xs.count(), 0);
    } 

    #[test]
    fn normal_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vec3::new(1.0, 0.0, 0.0));
    } 

    #[test]
    fn normal_on_y_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vec3::new(0.0, 1.0, 0.0));
    } 

    #[test]
    fn normal_on_z_axis() {
        let s = Sphere::default();
        let n = s.normal_at(Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vec3::new(0.0, 0.0, 1.0));
    } 

    #[test]
    fn normal_at_nonaxial_point() {
        let s = Sphere::default();
        let n = s.normal_at(Vec3::new(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt()/3.0));
        assert!(n.abs_diff_eq(Vec3::new(3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0, 3.0_f32.sqrt() / 3.0), EPSILON));
    } 

    #[test]
    fn the_normal_is_normalized() {
        let s = Sphere::default();
        let n = s.normal_at(Vec3::new(3.3_f32.sqrt() / 3.0, 3.3_f32.sqrt()/3.0, 3.3_f32.sqrt()/3.0));
        assert!(n.abs_diff_eq(n.normalize(), EPSILON));
    } 

    #[test]
    fn computing_normal_on_a_translated_sphere() {
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_translation(0.0, 1.0, 0.0);
        let n = o.normal_at(Vec3::new(0.0, 1.70711, -0.70711));
        assert!(n.abs_diff_eq(Vec3::new(0.0, 0.70711, -0.70711), EPSILON));
    } 

    #[test]
    fn computing_normal_on_a_transformed_sphere() {
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_rotation_z(PI / 5.0)
            .with_scale(1.0, 0.5, 1.0);
            
        let n = o.normal_at(Vec3::new(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0));
        assert!(n.abs_diff_eq(Vec3::new(0.0, 0.97014, -0.24254), EPSILON));
    } 
}