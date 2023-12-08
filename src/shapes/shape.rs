use crate::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeRef<'a> {
    Sphere(&'a Sphere),
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f32>;
}