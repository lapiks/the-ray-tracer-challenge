use glam::Vec3;
use crate::ray::Ray;
use super::shape::Shape;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let sphere_to_ray = ray.origin - self.position;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - self.radius * self.radius;
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
}

impl Default for Sphere {
    fn default() -> Self {
        Self { 
            position: Vec3::ZERO, 
            radius: 1.0 
        }
    }
}

impl Sphere {
    pub fn new(position: &Vec3, radius: f32) -> Self {
        Self {
            position: *position,
            radius
        }
    }
}


#[cfg(test)]
mod tests {
    use glam::Mat4;

    use crate::{object::Object, shapes::shape::ShapeRef};

    use super::*;

    // Creating a sphere
    #[test]
    fn new_sphere() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let radius = 1.0;
        let s = Sphere::new(&position, radius);
        assert_eq!(s.position, position);
        assert_eq!(s.radius, radius);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            &Vec3::new(0.0, 0.0, -5.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            &Vec3::new(0.0, 1.0, -5.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(
            &Vec3::new(0.0, 2.0, -5.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }
    
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            &Vec3::new(0.0, 0.0, 0.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            &Vec3::new(0.0, 0.0, 5.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::new(&Vec3::new(0.0, 0.0, 0.0), 1.0);
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            &Vec3::new(0.0, 0.0, 5.0), 
            &Vec3::new(0.0, 0.0, 1.0)
        );
        let s = Sphere::default();
        let mut o = Object::new(ShapeRef::Sphere(&s));
        o.set_transform(&Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0)));
        let xs = o.intersect(&r);
    } 
}