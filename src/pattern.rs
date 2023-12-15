use glam::{DVec3, DMat4};

use crate::{Color, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct PatternObject {
    pattern: Pattern,
    transform: DMat4,
    inverse_transform: DMat4,
}

impl PatternObject {
    pub fn new(pattern: Pattern) -> Self {
        Self {
            pattern,
            ..Default::default()
        }
    }

    pub fn with_transform(mut self, mat: &DMat4) -> Self {
        self.transform = *mat;
        self.inverse_transform = mat.inverse();
        self
    }

    pub fn color_at_object(&self, object: &Object, world_point: DVec3) -> Color {
        let object_point = object.inverse_transform().transform_point3(world_point);
        let pattern_point = self.inverse_transform.transform_point3(object_point);
        self.pattern.color_at(pattern_point)
    }
}

impl Default for PatternObject {
    fn default() -> Self {
        Self { 
            pattern: Pattern::Plain(PlainPattern::new(Color::white())), 
            transform: Default::default(), 
            inverse_transform: Default::default() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Plain(PlainPattern),
    Stripped(StrippedPattern),
}

pub trait PatternFunc {
    fn color_at(&self, point: DVec3) -> Color;
}

impl PatternFunc for Pattern {
    fn color_at(&self, point: DVec3) -> Color {
        match self {
            Pattern::Plain(p) => p.color_at(point),
            Pattern::Stripped(p) => p.color_at(point),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlainPattern {
    c: Color,
}

impl PlainPattern {
    pub fn new( c: Color) -> Self {
        Self {
            c
        }
    }

    pub fn color(&self) -> Color {
        self.c
    }
}

impl Default for PlainPattern {
    fn default() -> Self {
        Self::new(Color::white())
    }
}

impl PatternFunc for PlainPattern {
    fn color_at(&self, _: DVec3) -> Color {
        self.c
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrippedPattern {
    c0: Color,
    c1: Color,
}

impl StrippedPattern {
    pub fn new( c0: Color, c1: Color) -> Self {
        Self {
            c0, c1
        }
    }
}

impl PatternFunc for StrippedPattern {
    fn color_at(&self, point: DVec3) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.c0
        } else {
            self.c1
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{Object, shapes::{Shape, Sphere}};

    use super::*;

    #[test]
    fn create_a_stripe_pattern() {
        let pattern = StrippedPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.c0, Color::white());
        assert_eq!(pattern.c1, Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StrippedPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(dvec3(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StrippedPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(dvec3(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StrippedPattern::new(Color::white(), Color::black());
        assert_eq!(pattern.color_at(dvec3(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.color_at(dvec3(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(dvec3(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(dvec3(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.color_at(dvec3(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(2.0, 2.0, 2.0);
        let pattern = PatternObject::new(
            Pattern::Stripped(
                StrippedPattern::new(Color::white(), Color::black())
            )
        );
        assert_eq!(pattern.color_at_object(&o, dvec3(1.5, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let o = Object::new(Shape::Sphere(Sphere::default()));
        let pattern = PatternObject::new(
            Pattern::Stripped(
                StrippedPattern::new(Color::white(), Color::black())
            )
        )
            .with_transform(&DMat4::from_scale(dvec3(2.0, 2.0, 2.0)));

        assert_eq!(pattern.color_at_object(&o, dvec3(1.5, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let o = Object::new(Shape::Sphere(Sphere::default()))
            .with_scale(2.0, 2.0, 2.0);
        let pattern = PatternObject::new(
            Pattern::Stripped(
                StrippedPattern::new(Color::white(), Color::black())
            )
        )
            .with_transform(&DMat4::from_translation(dvec3(0.5, 0.0, 0.0)));
        
        assert_eq!(pattern.color_at_object(&o, dvec3(2.5, 0.0, 0.0)), Color::white());
    }
}
