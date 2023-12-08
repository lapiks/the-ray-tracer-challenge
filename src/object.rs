use glam::Mat4;

use crate::{
    shapes::shape::{ShapeRef, Shape}, 
    ray::Ray, 
    intersection::{Intersections, Intersection}
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object<'a> {
    shape: ShapeRef<'a>,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl<'a> Object<'a> {
    pub fn new(shape: ShapeRef<'a>) -> Self {
        Self {
            shape,
            transform: Mat4::IDENTITY,
            inverse_transform: Mat4::IDENTITY
        }
    }

    pub fn get_transform(&self) -> &Mat4 {
        &self.transform
    }

    pub fn set_transform(&mut self, mat: &Mat4) {
        self.transform = *mat;
        self.inverse_transform = mat.inverse();
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.inverse_transform);
        let ts = self.shape.intersect(&transformed_ray);
        let mut xs = Intersections::with_capacity(ts.len());
        for t in ts {
            xs.push(
                &Intersection::new(
                    t,
                    &self
                )
            )
        }

        xs
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::shapes::sphere::Sphere;
    use super::*;

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::default();
        let o = Object::new(ShapeRef::Sphere(&s));
        assert_eq!(o.transform, Mat4::IDENTITY);
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let s = Sphere::default();
        let mut o = Object::new(ShapeRef::Sphere(&s));
        let t = Mat4::from_translation(Vec3::new(2.0, 3.0, 4.0));
        o.set_transform(&t);
        assert_eq!(o.transform, t);
    }
}