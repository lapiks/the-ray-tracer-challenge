use glam::DVec3;

use crate::{ray::Ray, intersection::Intersections, Object};
use super::{Sphere, test_shape::TestShape, Plane, Cube, Group, Triangle};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere), 
    Plane(Plane),
    Cube(Cube),
    Triangle(Triangle),
    Group(Group),
    TestShape(TestShape),
}

impl Shape {
    pub fn as_group(&self) -> Option<&Group> {
        match self {
            Shape::Group(g) => Some(g),
            _ => None
        }
    }
}

pub trait Hittable {
    fn intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Intersections<'a>;
    fn normal_at(&self, world_point: DVec3) -> DVec3;
}

impl Hittable for Shape {
    fn intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere(s) => s.intersect(ray, object),
            Shape::Plane(p) => p.intersect(ray, object),
            Shape::Cube(c) => c.intersect(ray, object),
            Shape::Triangle(t) => t.intersect(ray, object),
            Shape::Group(g) => g.intersect(ray, object),
            Shape::TestShape(s) => s.intersect(ray, object),
        }
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
        match self {
            Shape::Sphere(s) => s.normal_at(point),
            Shape::Plane(p) => p.normal_at(point),
            Shape::Cube(c) => c.normal_at(point),
            Shape::Triangle(t) => t.normal_at(point),
            Shape::Group(g) => g.normal_at(point),
            Shape::TestShape(s) => s.normal_at(point),
        }
    }
}