use crate::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Material {
    fn default() -> Self {
        Self { 
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
         }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Object, shapes::{Sphere, Shape}};

    use super::*;

    #[test]
    fn a_sphere_has_a_default_material() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(*o.get_material(), Material::default());
    }
}

