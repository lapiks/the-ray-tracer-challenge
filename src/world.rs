use crate::{object::Object, ray::Ray, Color};

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::default(),
        }
    }

    pub fn with_objects(mut self, objects: Vec<Object>) -> Self {
        self.objects = objects;
        self
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        for object in &self.objects {
            let intersections = object
                .intersect(ray)
                .sort();
            if let Some(hit) = intersections.hit() {
                return Color::red();
            }
            
        }

        Color::black()
    }
}