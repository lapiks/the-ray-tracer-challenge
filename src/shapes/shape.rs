use crate::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeRef<'a> {
    Sphere(&'a Sphere),
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}

impl Shape for ShapeRef<'_> {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        match self {
            ShapeRef::Sphere(s) => s.intersect(ray),
        }
    }
}