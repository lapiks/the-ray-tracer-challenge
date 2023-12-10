use glam::Vec3;

use crate::{Color, light::PointLight};

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

    pub fn with_color(mut self, color: &Color) -> Self {
        self.color = *color;
        self
    }

    pub fn lighting(&self, light: &PointLight, point: &Vec3, eyev: &Vec3, normal: &Vec3) -> Color {
        let effective_color = self.color * *light.intensity();
        let lightv = (*light.position() - *point).normalize();

        let ambient = effective_color * self.ambient;
        let mut diffuse = Color::black();
        let mut specular = Color::black();
        let l_dot_n = lightv.dot(*normal);
        if l_dot_n >= 0.0 {
            diffuse = effective_color * self.diffuse * l_dot_n;
            let reflectv = -lightv - *normal * 2.0 * -lightv.dot(*normal);
            let r_dot_e = reflectv.dot(*eyev);
            if r_dot_e > 0.0 {
                specular = *light.intensity() * self.specular * r_dot_e.powf(self.shininess);
            }
        }

        ambient + diffuse + specular
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
    use glam::vec3;

    use crate::{Object, shapes::{Sphere, Shape}, light::PointLight};

    use super::*;

    #[test]
    fn a_sphere_has_a_default_material() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(*o.get_material(), Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut o = Object::new(Shape::Sphere(Sphere::default()));
        let mut m = Material::default();
        m.ambient = 1.0;
        o.set_material(&m);
        assert_eq!(*o.get_material(), m);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = vec3(0.0, 0.0, 0.0);
        let eyev = vec3(0.0, 0.0, -1.0);
        let normalv = vec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            &vec3(0.0, 0.0, -10.0),
            &Color::white()
        );
        assert_eq!(m.lighting(&l, &position, &eyev, &normalv), Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::default();
        let position = vec3(0.0, 0.0, 0.0);
        let eyev = vec3(0.0, 2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = vec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            &vec3(0.0, 0.0, -10.0),
            &Color::white()
        );
        assert_eq!(m.lighting(&l, &position, &eyev, &normalv), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::default();
        let position = vec3(0.0, 0.0, 0.0);
        let eyev = vec3(0.0, 0.0, -1.0);
        let normalv = vec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            &vec3(0.0, 10.0, -10.0),
            &Color::white()
        );
        assert_eq!(m.lighting(&l, &position, &eyev, &normalv), Color::new(0.7364, 0.7364, 0.7364));
    }
    
    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = vec3(0.0, 0.0, 0.0);
        let eyev = vec3(0.0, -2.0_f32.sqrt() / 2.0, -2.0_f32.sqrt() / 2.0);
        let normalv = vec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            &vec3(0.0, 10.0, -10.0),
            &Color::white()
        );
        assert_eq!(m.lighting(&l, &position, &eyev, &normalv), Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = vec3(0.0, 0.0, 0.0);
        let eyev = vec3(0.0, 0.0, -1.0);
        let normalv = vec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            &vec3(0.0, 0.0, 10.0),
            &Color::white()
        );
        assert_eq!(m.lighting(&l, &position, &eyev, &normalv), Color::new(0.1, 0.1, 0.1));
    }
}

