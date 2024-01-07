use glam::DVec3;

use crate::{Color, World};

use super::{PointLight, AreaLight};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Light {
    PointLight(PointLight),
    AreaLight(AreaLight),
}

pub trait LightSource {
    fn position(&self) -> DVec3;
    fn intensity(&self) -> Color;
    fn intensity_at(&self, world_point: DVec3, world: &World) -> f64;
}

impl LightSource for Light {
    fn position(&self) -> DVec3 {
        match self {
            Light::PointLight(l) => l.position(),
            Light::AreaLight(l) => l.position(),
        }
    }

    fn intensity(&self) -> Color {
        match self {
            Light::PointLight(l) => l.intensity(),
            Light::AreaLight(l) => l.intensity(),
        }
    }

    fn intensity_at(&self, world_point: DVec3, world: &World) -> f64 {
        match self {
            Light::PointLight(l) => l.intensity_at(world_point, world),
            Light::AreaLight(l) => l.intensity_at(world_point, world),
        }
    }
}