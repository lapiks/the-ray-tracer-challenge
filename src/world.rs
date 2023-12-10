use crate::{object::Object, ray::Ray, Color, PointLight};

pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::default(),
            lights: Vec::default(),
        }
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
        for object in &self.objects {
            let intersections = object
                .intersect(ray)
                .sort();
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
}