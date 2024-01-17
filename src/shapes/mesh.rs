use glam::DVec3;

use crate::{ray::Ray, Object, intersection::Intersections, bounds::BoundingBox};
use super::{shape::Hittable, Group};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mesh {
    triangle_group: Group,
}

impl Hittable for Mesh {
    fn intersect<'a>(&'a self, ray: &Ray, this: &'a Object) -> Intersections<'a> {
        self.triangle_group.intersect(ray, this)
    }

    fn normal_at(&self, _: DVec3, _: f64, _: f64) -> DVec3 {
        DVec3::default()
    }

    fn bounds(&self) -> BoundingBox {
        self.triangle_group.bounds()
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_triangles(mut self, triangles: Vec<Object>) -> Self {
        self.triangle_group = Group::new().with_objects(triangles);
        self
    }

    pub fn triangles(&mut self) -> &Vec<Object> {
        self.triangle_group.objects()
    }

    pub fn triangles_mut(&mut self) -> &mut Vec<Object> {
        self.triangle_group.objects_mut()
    }

    pub fn divide(&mut self, threshold: usize) {
        self.triangle_group.divide(threshold);
    }
}

#[cfg(test)]
mod tests {

}