use glam::{DMat4, DVec3};

use crate::{
    shapes::shape::{Shape, Hittable}, 
    ray::Ray, 
    intersection::Intersections, 
    material::Material, transformations::{Transform, TransformBuilder, Transformable}
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transform: Transform,
    shadow: bool,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            material: Material::default(),
            transform: Transform::default(),
            shadow: true,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_translation(self, x: f64, y: f64, z: f64) -> TransformBuilder<Object> {
        TransformBuilder::new(
            Transform::default(),
            self,
        )
        .with_translation(x, y, z)
    }

    pub fn with_scale(self, x: f64, y: f64, z: f64) -> TransformBuilder<Object> {
        TransformBuilder::new(
            Transform::default(),
            self,
        )
        .with_scale(x, y, z)
    }

    pub fn with_rotation_x(self, angle: f64) -> TransformBuilder<Object> {
        TransformBuilder::new(
            Transform::default(),
            self,
        )
        .with_rotation_x(angle)
    }

    pub fn with_rotation_y(self, angle: f64) -> TransformBuilder<Object> {
        TransformBuilder::new(
            Transform::default(),
            self,
        )
        .with_rotation_y(angle)
    }

    pub fn with_rotation_z(self, angle: f64) -> TransformBuilder<Object> {
        TransformBuilder::new(
            Transform::default(),
            self,
        )
        .with_rotation_z(angle)
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    pub fn shadow(&self) -> bool {
        self.shadow
    } 

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn inverse_transform(&self) -> &DMat4 {
        &self.transform.inverse_matrix
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let transformed_ray = ray.transform(&self.transform.inverse_matrix);
        self.shape.intersect(&transformed_ray, &self)
    }

    pub fn normal_at(&self, world_point: DVec3) -> DVec3 {
        let object_normal = self.shape.normal_at(self.transform.inverse_matrix.transform_point3(world_point));
        self.transform
            .inverse_matrix
            .transpose()
            .transform_vector3(object_normal)
            .normalize()
    }

    pub fn world_to_object(&self, world_point: DVec3) -> DVec3 {
        DVec3::default()
    } 
}

impl Transformable for Object {
    fn apply_transform(self, transform: Transform) -> Self {
        self.with_transform(transform)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::sphere::Sphere;
    use super::*;

    #[test]
    fn the_default_transformation() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(o.transform.matrix, DMat4::IDENTITY);
        assert_eq!(o.transform.inverse_matrix, DMat4::IDENTITY);
    }

    #[test]
    fn assigning_a_transformation() {
        let t = Transform::new().with_translation(2.0, 3.0, 4.0);
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_transform(t.clone());
        
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