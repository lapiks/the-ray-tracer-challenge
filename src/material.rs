use glam::DVec3;

use crate::{Color, light::PointLight, Pattern, pattern::{PlainPattern, PatternObject}, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pattern: PatternObject,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
    reflective: f64,
    transparency: f64,
    refractive_index: f64,
}

impl Material {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pattern(mut self, pattern: PatternObject) -> Self {
        self.pattern = pattern;
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

    pub fn with_reflective(mut self, reflective: f64) -> Self {
        self.reflective = reflective;
        self
    }

    pub fn with_transparency(mut self, transparency: f64) -> Self {
        self.transparency = transparency;
        self
    }

    pub fn with_refractive_index(mut self, refractive_index: f64) -> Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn set_pattern(&mut self, pattern: PatternObject) -> &mut Self {
        self.pattern = pattern;
        self
    }

    pub fn set_ambient(&mut self, ambient: f64) -> &mut Self {
        self.ambient = ambient;
        self
    }

    pub fn set_diffuse(&mut self, diffuse: f64) -> &mut Self {
        self.diffuse = diffuse;
        self
    }

    pub fn set_specular(&mut self, specular: f64) -> &mut Self {
        self.specular = specular;
        self
    }

    pub fn set_shininess(&mut self, shininess: f64) -> &mut Self {
        self.shininess = shininess;
        self
    }

    pub fn set_reflective(&mut self, reflective: f64) -> &mut Self {
        self.reflective = reflective;
        self
    }

    pub fn set_transparency(&mut self, transparency: f64) -> &mut Self {
        self.transparency = transparency;
        self
    }

    pub fn set_refractive_index(&mut self, refractive_index: f64) -> &mut Self {
        self.refractive_index = refractive_index;
        self
    }

    pub fn pattern(&self) -> &PatternObject {
        &self.pattern
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

    pub fn reflective(&self) -> f64 {
        self.reflective
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn refractive_index(&self) -> f64 {
        self.refractive_index
    }

    pub fn lighting(&self, object: &Object, light: &PointLight, point: DVec3, eyev: DVec3, normal: DVec3, intensity: f64) -> Color {
        let effective_color = self.pattern.color_at_object(object, point) * light.intensity();
        let ambient = effective_color * self.ambient;

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

        ambient + diffuse * intensity + specular * intensity
    }
}

impl Default for Material {
    fn default() -> Self {
        Self { 
            pattern: PatternObject::new(Pattern::Plain(PlainPattern::new(Color::white()))),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
         }
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{Object, shapes::{Sphere, Shape}, light::PointLight, pattern::StrippedPattern, world::tests::default_world};

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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                position, 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::new(1.9, 1.9, 1.9)
        );
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())), 
                &l, 
                position, 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::new(1.0, 1.0, 1.0)
        );
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                position, 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::new(0.7364, 0.7364, 0.7364)
        );
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                position, 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::new(1.6364, 1.6364, 1.6364)
        );
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                position, 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::new(0.1, 0.1, 0.1)
        );
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                position, 
                eyev, 
                normalv, 
                0.0
            ), 
            Color::new(0.1, 0.1, 0.1)
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::default()
            .with_pattern(
                PatternObject::new(
                    Pattern::Stripped(StrippedPattern::new(Color::white(), Color::black()))
                )
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
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                dvec3(0.9, 0.0, 0.0), 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::white()
        );
        assert_eq!(
            m.lighting(
                &Object::new(Shape::Sphere(Sphere::default())),
                &l, 
                dvec3(1.1, 0.0, 0.0), 
                eyev, 
                normalv, 
                1.0
            ), 
            Color::black()
        );
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let m = Material::default();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::default();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }

    #[test]
    fn lighting_uses_light_intensity_to_attenuate_color() {
        let mut objects = default_world().objects().clone();
        let first_object = &objects[0];
        objects[0] = first_object.clone()
            .with_material(
                first_object.material().clone()
                .with_ambient(0.1)
                .with_diffuse(0.9)
                .with_specular(0.0)
                .with_pattern(
                    PatternObject::new(Pattern::Plain(PlainPattern::new(Color::white())))
                )
            );

        let w = default_world()
        .with_lights(vec![
            PointLight::new(dvec3(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0))
        ])
        .with_objects(objects);

        let object = w.object(0).unwrap();
        let light = w.light(0).unwrap();

        let pt = dvec3(0.0, 0.0, -1.0);
        let eyev = dvec3(0.0, 0.0, -1.0);
        let normalv = dvec3(0.0, 0.0, -1.0);

        assert_eq!(object.material().lighting(object, light, pt, eyev, normalv, 1.0), Color::white());
        assert_eq!(object.material().lighting(object, light, pt, eyev, normalv, 0.5), Color::new(0.55, 0.55, 0.55));
        assert_eq!(object.material().lighting(object, light, pt, eyev, normalv, 0.0), Color::new(0.1, 0.1, 0.1));
    }

}

