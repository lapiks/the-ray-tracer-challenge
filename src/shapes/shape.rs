use glam::DVec3;

use crate::{ray::Ray, intersection::Intersections, Object, bounds::Bounds};
use super::{Sphere, test_shape::TestShape, Plane, Cube, Group, Triangle, SmoothTriangle, Mesh};

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Sphere(Sphere), 
    Plane(Plane),
    Cube(Cube),
    Triangle(Triangle),
    SmoothTriangle(SmoothTriangle),
    Mesh(Mesh),
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
    fn normal_at(&self, world_point: DVec3, u: f64, v: f64) -> DVec3;
    fn bounds(&self) -> Bounds;
}

impl Hittable for Shape {
    fn intersect<'a>(&'a self, ray: &Ray, object: &'a Object) -> Intersections<'a> {
        match self {
            Shape::Sphere(s) => s.intersect(ray, object),
            Shape::Plane(p) => p.intersect(ray, object),
            Shape::Cube(c) => c.intersect(ray, object),
            Shape::Triangle(t) => t.intersect(ray, object),
            Shape::SmoothTriangle(t) => t.intersect(ray, object),
            Shape::Mesh(m) => m.intersect(ray, object),
            Shape::Group(g) => g.intersect(ray, object),
            Shape::TestShape(s) => s.intersect(ray, object),
        }
    }

    fn normal_at(&self, point: DVec3, u: f64, v: f64) -> DVec3 {
        match self {
            Shape::Sphere(s) => s.normal_at(point, u, v),
            Shape::Plane(p) => p.normal_at(point, u, v),
            Shape::Cube(c) => c.normal_at(point, u, v),
            Shape::Triangle(t) => t.normal_at(point, u, v),
            Shape::SmoothTriangle(t) => t.normal_at(point, u, v),
            Shape::Mesh(m) => m.normal_at(point, u, v),
            Shape::Group(g) => g.normal_at(point, u, v),
            Shape::TestShape(s) => s.normal_at(point, u, v),
        }
    }

    fn bounds(&self) -> Bounds {
        match self {
            Shape::Sphere(s) => s.bounds(),
            Shape::Plane(p) => p.bounds(),
            Shape::Cube(c) => c.bounds(),
            Shape::Triangle(t) => t.bounds(),
            Shape::SmoothTriangle(t) => t.bounds(),
            Shape::Mesh(m) => m.bounds(),
            Shape::Group(g) => g.bounds(),
            Shape::TestShape(s) => s.bounds(),
        }
    }
}