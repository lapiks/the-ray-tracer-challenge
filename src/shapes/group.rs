use glam::DVec3;

use crate::{ray::Ray, Object, intersection::Intersections};
use super::shape::Hittable;

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    objects: Vec<Object>,
}

impl Hittable for Group {
    fn intersect<'a>(&self, _: &Ray, _: &'a Object) -> Intersections<'a> {
        let xs = Intersections::new();
        xs 
    }

    fn normal_at(&self, _: DVec3) -> DVec3 {
        DVec3::default()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            objects: Vec::default()
        }
    }
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

#[cfg(test)]
mod tests {
    use glam::DMat4;
    use crate::shapes::{Shape, Sphere};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::default();
        let o = Object::new(Shape::Group(g.clone()));
        assert_eq!(*o.transform(), DMat4::IDENTITY);
        assert_eq!(g.objects.len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::default();
        let s = Object::new(Shape::Sphere(Sphere::default()));
        g.add_object(s.clone());
        assert_eq!(g.objects.len(), 1);
        assert_eq!(g.objects[0], s);
    }
}