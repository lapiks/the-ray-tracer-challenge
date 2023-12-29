use glam::DVec3;

use crate::ray::Ray;
use super::{sphere::Sphere, test_shape::TestShape, plane::Plane, cube::Cube, Group};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere), 
    Plane(Plane),
    Cube(Cube),
    Group(Group),
    TestShape(TestShape),
}

pub trait Hittable {
    fn intersect(&self, ray: &Ray) -> Vec<f64>;
    fn normal_at(&self, world_point: DVec3) -> DVec3;
}

impl Hittable for Shape {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Sphere(s) => s.intersect(ray),
            Shape::Plane(p) => p.intersect(ray),
            Shape::Cube(c) => c.intersect(ray),
            Shape::Group(g) => g.intersect(ray),
            Shape::TestShape(s) => s.intersect(ray),
        }
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
            Shape::Plane(p) => p.normal_at(point),
            Shape::Cube(c) => c.normal_at(point),
            Shape::Group(g) => g.normal_at(point),
            Shape::TestShape(s) => s.normal_at(point),
        }
    }
}