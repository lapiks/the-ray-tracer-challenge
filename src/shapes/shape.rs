use glam::DVec3;

use crate::ray::Ray;
use super::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Sphere(Sphere),
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, world_point: DVec3) -> DVec3;
}

impl Hittable for Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
        }
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
        }
    }
}