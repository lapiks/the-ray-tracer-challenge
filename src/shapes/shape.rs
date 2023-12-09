use crate::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}

impl Hittable for Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
        }
    }
}