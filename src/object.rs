use glam::{DMat4, DVec3};

use crate::{
    shapes::shape::{Shape, Hittable}, 
    ray::Ray, 
    intersection::{Intersections, Intersection}, material::Material
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transform: DMat4,
    inverse_transform: DMat4,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            material: Material::default(),
            transform: DMat4::IDENTITY,
            inverse_transform: DMat4::IDENTITY
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_transform(mut self, transform: &DMat4) -> Self {
        self.transform = *transform;
        self.inverse_transform = transform.inverse();
        self
    }

    pub fn with_translation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transform = DMat4::from_translation(DVec3::new(x, y, z)) * self.transform;
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transform = DMat4::from_scale(DVec3::new(x, y, z)) * self.transform;
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_x(mut self, angle: f64) -> Self {
        self.transform = DMat4::from_rotation_x(angle) * self.transform;
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_y(mut self, angle: f64) -> Self {
        self.transform = DMat4::from_rotation_y(angle) * self.transform;
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn with_rotation_z(mut self, angle: f64) -> Self {
        self.transform = DMat4::from_rotation_z(angle) * self.transform;
        self.inverse_transform = self.transform.inverse();
        self
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn transform(&self) -> &DMat4 {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &DMat4 {
        &self.inverse_transform
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

    pub fn normal_at(&self, world_point: DVec3) -> DVec3 {
        let object_normal = self.shape.normal_at(self.inverse_transform.transform_point3(world_point));
        self.transform
            .inverse()
            .transpose()
            .transform_vector3(object_normal)
            .normalize()
    }
}

#[cfg(test)]
mod tests {
    use glam::DVec3;

    use crate::shapes::sphere::Sphere;
    use super::*;

    #[test]
    fn the_default_transformation() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(o.transform, DMat4::IDENTITY);
    }

    #[test]
    fn assigning_a_transformation() {
        let t = DMat4::from_translation(DVec3::new(2.0, 3.0, 4.0));
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_transform(&t);
        
        assert_eq!(o.transform, t);
    }

    #[test]
    fn the_default_material() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(o.material, Material::default());
    }

    #[test]
    fn assigning_a_material() {
        let m = Material::default()
            .with_ambient(1.0);
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m.clone());
        
        assert_eq!(o.material, m);
    }
}