use crate::{object::Object, ray::Ray, Color, PointLight, intersection::Intersections};

pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        Self { 
            objects: Vec::default(), 
            lights: Vec::default() 
        }
    }
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;
        self
    }

    pub fn with_lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = lights;
        self
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersects(ray);

        for object in &self.objects {
            if let Some(hit) = intersections.hit() {
                for light in &self.lights {
                    let hit_point = ray.at(hit.t());
                    let normal = object.normal_at(&hit_point);
                    return object.get_material().lighting(&light, &hit_point, &-ray.direction, &normal);
                }   
            }
        }

        Color::black()
    }

    fn intersects(&self, ray: &Ray) -> Intersections{
        let mut intersections = Intersections::new_empty();
        for object in &self.objects {
            intersections
                .append(
                    object.intersect(ray)
                );
                
        }

        intersections.sort()
    } 
}

#[cfg(test)]
mod tests {
    use glam::{Vec3, vec3};

    use crate::{shapes::{Sphere, Shape}, Material};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::default();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    pub fn default_world() -> World {
        let l = PointLight::new(
            &Vec3::new(-10.0, 10.0, -10.0), 
            &Color::white()
        );
        let m = Material::new()
            .with_color(&Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5);
        World::default()
            .with_objects(vec![s1, s2])
            .with_lights(vec![l])
    }

    #[test]
    fn the_default_world() {
        let w = default_world();

        let l = PointLight::new(
            &Vec3::new(-10.0, 10.0, -10.0), 
            &Color::white()
        );
        let m = Material::new()
            .with_color(&Color::new(0.8, 1.0, 0.6))
            .with_diffuse(0.7)
            .with_specular(0.2);
        let s1 = Object::new(Shape::Sphere(Sphere::default()))
            .with_material(m);
        let s2 = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(0.5, 0.5, 0.5);

        assert_eq!(w.objects.len(), 2);
        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.lights[0], l);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            &vec3(0.0, 0.0, -5.0),
            &vec3(0.0, 0.0, 1.0)
        );
        let xs = w.intersects(&r);
        assert_eq!(xs.count(), 4);
        assert_eq!(xs.get(0).unwrap().t(), 4.0);
        assert_eq!(xs.get(1).unwrap().t(), 4.5);
        assert_eq!(xs.get(2).unwrap().t(), 5.5);
        assert_eq!(xs.get(3).unwrap().t(), 6.0);
    }
}