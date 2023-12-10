use glam::{Mat4, Vec3};

use crate::{
    shapes::shape::{Shape, Hittable}, 
    ray::Ray, 
    intersection::{Intersections, Intersection}, material::Material
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            material: Material::default(),
            transform: Mat4::IDENTITY,
            inverse_transform: Mat4::IDENTITY
        }
    }

    pub fn with_translation(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform *= Mat4::from_translation(Vec3::new(x, y, z));
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.transform *= Mat4::from_scale(Vec3::new(x, y, z));
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_x(mut self, angle: f32) -> Self {
        self.transform *= Mat4::from_rotation_x(angle);
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_y(mut self, angle: f32) -> Self {
        self.transform *= Mat4::from_rotation_y(angle);
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_z(mut self, angle: f32) -> Self {
        self.transform *= Mat4::from_rotation_z(angle);
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn get_material(&self) -> &Material {
        &self.material
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

    pub fn normal_at(&self, world_point: &Vec3) -> Vec3 {
        let object_normal = self.shape.normal_at(&self.inverse_transform.transform_point3(*world_point));
        self.transform
            .inverse()
            .transpose()
            .transform_vector3(object_normal)
            .normalize()
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
        let o = Object::new(Shape::Sphere(s));
        assert_eq!(o.transform, Mat4::IDENTITY);
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let s = Sphere::default();
        let mut o = Object::new(Shape::Sphere(s));
        let t = Mat4::from_translation(Vec3::new(2.0, 3.0, 4.0));
        o.set_transform(&t);
        assert_eq!(o.transform, t);
    }
}