use glam::DVec3;

use crate::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    PlainPattern(PlainPattern),
    StrippedPattern(StrippedPattern),
}

pub trait PatternFunc {
    fn color_at(&self, point: DVec3) -> Color;
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
}
