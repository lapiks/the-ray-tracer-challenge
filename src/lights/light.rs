use glam::DVec3;

use crate::{Color, World};

use super::PointLight;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Light {
    PointLight(PointLight),
}

pub trait LightSource {
    fn position(&self) -> DVec3;
    fn color(&self) -> Color;
    fn intensity_at(&self, world_point: DVec3, world: &World) -> f64;
}

impl LightSource for Light {
    fn position(&self) -> DVec3 {
        match self {
            Light::PointLight(l) => l.position(),
        }
    }

    fn color(&self) -> Color {
        match self {
            Light::PointLight(l) => l.color(),
        }
    }

    fn intensity_at(&self, world_point: DVec3, world: &World) -> f64 {
        match self {
            Light::PointLight(l) => l.intensity_at(world_point, world),
        }
    }
}