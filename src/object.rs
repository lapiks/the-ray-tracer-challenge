use glam::{DMat4, DVec3};

use crate::{
    shapes::shape::{Shape, Hittable}, 
    ray::Ray, 
    intersection::Intersections, 
    material::Material, transformations::{Transform, TransformBuilder, Transformable}, bounds::Bounds
};

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    shape: Shape,
    material: Material,
    transform: Transform,
    shadow: bool,
    bounds: Bounds,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        let bounds = shape.bounds();        
        Self {
            shape,
            material: Material::default(),
            transform: Transform::default(),
            shadow: true,
            bounds,
        }
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material.clone();
        self.shape = match self.shape {
            Shape::Mesh(mut m) => {
                let mut new_triangles = Vec::with_capacity(m.triangles().len());
                for triangle in m.triangles().iter() {
                    new_triangles.push(triangle.clone().with_material(material.clone()));
                }
                m = m.with_triangles(new_triangles);
                Shape::Mesh(m)
            }
            _ => { self.shape }
        };
        self
    }

    pub fn with_shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.bounds = self.shape.bounds().transform(&transform.matrix);
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

    pub fn shape(&self) -> &Shape {
        &self.shape
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

    pub fn bounds(&self) -> &Bounds {
        &self.bounds
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let local_ray = ray.transform(&self.transform.inverse_matrix);
        self.shape.intersect(&local_ray, &self)
    }

    pub fn normal_at(&self, world_point: DVec3, u: f64, v: f64) -> DVec3 {
        let object_normal = self.shape.normal_at(self.world_to_object(world_point), u, v);
        self.normal_to_world(object_normal)
    }

    pub fn world_to_object(&self, world_point: DVec3) -> DVec3 {
        self.transform.inverse_matrix.transform_point3(world_point)
    }

    fn normal_to_world(&self, normal: DVec3) -> DVec3 {
        self.transform.inverse_transpose_matrix.transform_vector3(normal).normalize()
    }

    fn set_transform(&mut self, transform: Transform) {
        self.bounds = self.shape().bounds().transform(&transform.matrix);
        self.transform = transform;
    }
}

impl Transformable for Object {
    fn apply_transform(&mut self, transform: Transform) {
        match &mut self.shape {
            Shape::Group(g) => {
                for object in g.objects_mut() {
                    object.apply_transform(transform.clone());
                }
                self.set_transform(Transform::identity());
            },
            Shape::Mesh(m) => {
                for object in m.triangles_mut() {
                    object.apply_transform(transform.clone());
                }
                self.set_transform(Transform::identity());
            },
            _other => {
                let new_transform = self.transform.clone().apply(transform);
                self.set_transform(new_transform);
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    const EPSILON: f64 = 0.0001;

    use glam::dvec3;

    use crate::shapes::{sphere::Sphere, Group};
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

    #[test]
    fn convert_a_point_from_world_to_object_space() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 0.0, 0.0)
        .transform();

        let g2 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s])
            )
        )
        .with_scale(2.0, 2.0, 2.0)
        .transform();

        let g1 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![g2])
            )
        )
        .with_rotation_y(PI / 2.0)
        .transform();

        let retrieved_s = &g1.shape()
        .as_group()
        .unwrap()
        .objects()[0]
        .shape()
        .as_group()
        .unwrap()
        .objects()[0];

        assert!(retrieved_s.world_to_object(dvec3(-2.0, 0.0, -10.0)).abs_diff_eq(dvec3(0.0, 0.0, -1.0), EPSILON));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 0.0, 0.0)
        .transform();

        let g2 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s])
            )
        )
        .with_scale(1.0, 2.0, 3.0)
        .transform();

        let g1 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![g2])
            )
        )
        .with_rotation_y(PI / 2.0)
        .transform();

        let retrieved_s = &g1.shape()
        .as_group()
        .unwrap()
        .objects()[0]
        .shape()
        .as_group()
        .unwrap()
        .objects()[0];

        assert!(
            retrieved_s.normal_to_world(
                dvec3(f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0, f64::sqrt(3.0)/3.0)
            )
            .abs_diff_eq(dvec3(0.2857, 0.4286, -0.8571), EPSILON)
        );
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let s = Object::new(Shape::Sphere(Sphere::default()))
        .with_translation(5.0, 0.0, 0.0)
        .transform();

        let g2 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![s])
            )
        )
        .with_scale(1.0, 2.0, 3.0)
        .transform();

        let g1 = Object::new(
            Shape::Group(
                Group::new()
                .with_objects(vec![g2])
            )
        )
        .with_rotation_y(PI / 2.0)
        .transform();

        let retrieved_s = &g1.shape()
        .as_group()
        .unwrap()
        .objects()[0]
        .shape()
        .as_group()
        .unwrap()
        .objects()[0];

        assert!(retrieved_s.normal_at(dvec3(1.7321, 1.1547, -5.5774), 0.0, 0.0).abs_diff_eq(dvec3(0.2857, 0.4286, -0.8571), EPSILON));
    }
}