use glam::DVec3;

use crate::{Color, light::PointLight, Pattern, pattern::PlainPattern};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    pattern: Pattern,
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_ambient(mut self, ambient: f64) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f64) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f64) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_shininess(mut self, shininess: f64) -> Self {
        self.shininess = shininess;
        self
    }

    pub fn with_pattern(mut self, pattern: Pattern) -> Self {
        self.pattern = pattern;
        self
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn ambient(&self) -> f64 {
        self.ambient
    }

    pub fn diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn specular(&self) -> f64 {
        self.specular
    }
    pub fn shininess(&self) -> f64 {
        self.shininess
    }

    pub fn lighting(&self, light: &PointLight, point: DVec3, eyev: DVec3, normal: DVec3, is_in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity();
        let ambient = effective_color * self.ambient;
        
        if is_in_shadow {
            return ambient;
        }

        let mut diffuse = Color::black();
        let mut specular = Color::black();

        let lightv = (light.position() - point).normalize();
        let l_dot_n = lightv.dot(normal);

        if l_dot_n >= 0.0 {
            diffuse = effective_color * self.diffuse * l_dot_n;
            let reflectv = -lightv - normal * 2.0 * -lightv.dot(normal);
            let r_dot_e = reflectv.dot(eyev);
            if r_dot_e > 0.0 {
                specular = light.intensity() * self.specular * r_dot_e.powf(self.shininess);
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
            shininess: 200.0,
            pattern: Pattern::PlainPattern(PlainPattern::new(Color::white())),
         }
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{Object, shapes::{Sphere, Shape}, light::PointLight, pattern::StrippedPattern};

    use super::*;

    #[test]
    fn a_sphere_has_a_default_material() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        assert_eq!(*o.material(), Material::default());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let m = Material::default()
            .with_ambient(1.0);
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m.clone());
        assert_eq!(*o.material(), m);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 0.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, false), Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 0.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, false), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 10.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, false), Color::new(0.7364, 0.7364, 0.7364));
    }
    
    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 10.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, false), Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 0.0, 10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, false), Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_light_in_shadow() {
        let m = Material::default();
        let position = dvec3(0.0, 0.0, 0.0);
        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 0.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, position, eyev, normalv, true), Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::default()
            .with_pattern(
                Pattern::StrippedPattern(StrippedPattern::new(Color::white(), Color::black()))
            )
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0);

        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);
        let l = PointLight::new(
            dvec3(0.0, 0.0, -10.0),
            Color::white()
        );
        assert_eq!(m.lighting(&l, dvec3(0.9, 0.0, 0.0), eyev, normalv, true), Color::white());
        assert_eq!(m.lighting(&l, dvec3(1.1, 0.0, 0.0), eyev, normalv, true), Color::black());
    }
}

