use glam::DVec3;

use crate::{Color, World};

use super::{PointLight, AreaLight};

#[derive(Debug, Clone, PartialEq)]
pub enum Light {
    PointLight(PointLight),
    AreaLight(AreaLight),
}

pub trait LightSource {
    fn positions(&self) -> &[DVec3];
    fn intensity(&self) -> Color;
    fn intensity_at(&self, world_point: DVec3, world: &World) -> f64;
}

impl LightSource for Light {
    fn positions(&self) -> &[DVec3] {
        match self {
            Light::PointLight(l) => l.positions(),
            Light::AreaLight(l) => l.positions(),
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