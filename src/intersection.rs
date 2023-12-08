use crate::shapes::shape::ShapeRef;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    t: f32,
    object: ShapeRef<'a>
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: ShapeRef<'a>) -> Self {
        Self {
            t,
            object,
        }
    }
}

pub struct Intersections<'a> {
    intersections: Vec<Intersection<'a>>,
}

impl<'a> Intersections<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        Self {  
            intersections
        }
    }

    pub fn sort(&mut self) {
        //self.intersections.sort();
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.intersections.iter().find(|i| i.t >= 0.0)
    }

    pub fn count(&self) -> usize {
        self.intersections.len()
    }

    pub fn get(&self, index: usize) -> Option<&Intersection> {
        self.intersections.get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::sphere::Sphere;

    use super::*;

    // An intersection encapsulate t and object
    #[test]
    fn new_intersection() {
        let s = Sphere::default();
        let i = Intersection::new(3.5, ShapeRef::Sphere(&s));
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, ShapeRef::Sphere(&s));
    }

    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, ShapeRef::Sphere(&s));
        let i2 = Intersection::new(2.0, ShapeRef::Sphere(&s));
        let xs = Intersections::new(vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get(0), Some(&i1));
        assert_eq!(xs.get(1), Some(&i2));
    }
}