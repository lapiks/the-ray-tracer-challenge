use glam::DVec3;

use crate::{ray::Ray, Object};
use super::shape::Hittable;

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    objects: Vec<Object>,
}

impl Hittable for Group {
    fn intersect(&self, ray: &Ray) -> Vec<f64> {
        vec![]
    }

    fn normal_at(&self, point: DVec3) -> DVec3 {
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
}

#[cfg(test)]
mod tests {
    use glam::{DVec3, DMat4};
    use crate::shapes::Shape;

    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn creating_a_new_group() {
        let g = Group::default();
        let o = Object::new(Shape::Group(g.clone()));
        assert_eq!(*o.transform(), DMat4::IDENTITY);
        assert_eq!(g.objects.len(), 0);
    }
}