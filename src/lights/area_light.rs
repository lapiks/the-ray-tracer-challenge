use glam::DVec3;

use crate::Color;

use super::light::LightSource;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AreaLight {
    corner: DVec3,
    uvec: DVec3,
    usteps: usize,
    vvec: DVec3,
    vsteps: usize,
    intensity: Color,
    samples: usize,
    position: DVec3,
}

impl AreaLight {
    pub fn new(
        corner: DVec3, 
        full_uvec: DVec3, 
        usteps: usize,
        full_vvec: DVec3,
        vsteps: usize,
        color: Color) 
    -> Self {
        Self {
            corner,
            uvec: full_uvec / usteps as f64,
            usteps,
            vvec: full_vvec / vsteps as f64,
            vsteps,
            intensity: color,
            samples: usteps * vsteps,
            position: corner + full_uvec / 2.0 + full_vvec / 2.0,
        }
    }

    // returns the point in the middle of the cell at the given coordinates
    fn point_on_light(&self, u: usize, v: usize) -> DVec3{
        self.corner + 
        self.uvec * (u as f64 + 0.5) +
        self.vvec * (v as f64 + 0.5) 
    }
}

impl LightSource for AreaLight {
    fn position(&self) -> DVec3 {
        self.position
    }

    fn intensity(&self) -> crate::Color {
        self.intensity
    }

    fn intensity_at(&self, world_point: DVec3, world: &crate::World) -> f64 {
        let mut total = 0.0;
        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                total += match world.is_shadowed(
                    world_point,
                    self.point_on_light(u, v)
                ) {
                    true => 0.0,
                    false => 1.0,
                }
            }
        }
        total / self.samples as f64
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::world::tests::default_world;

    use super::*;

    #[test]
    fn creating_an_area_light() {
        let light = AreaLight::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(2.0, 0.0, 0.0),
            4,
            dvec3(0.0, 0.0, 1.0),
            2,
            Color::white()
        );

        assert_eq!(light.corner, dvec3(0.0, 0.0, 0.0));
        assert_eq!(light.uvec, dvec3(0.5, 0.0, 0.0));
        assert_eq!(light.usteps, 4);
        assert_eq!(light.vvec, dvec3(0.0, 0.0, 0.5));
        assert_eq!(light.vsteps, 2);
        assert_eq!(light.intensity, Color::white());
        assert_eq!(light.samples, 8);
        assert_eq!(light.position, dvec3(1.0, 0.0, 0.5));
    }

    #[test]
    fn finding_a_single_point_on_an_area_light() {
        let light = AreaLight::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(2.0, 0.0, 0.0),
            4,
            dvec3(0.0, 0.0, 1.0),
            2,
            Color::white()
        );

        let u_v_result = vec![
            (0, 0, dvec3(0.25, 0.0, 0.25)),
            (1, 0, dvec3(0.75, 0.0, 0.25)),
            (0, 1, dvec3(0.25, 0.0, 0.75)),
            (2, 0, dvec3(1.25, 0.0, 0.25)),
            (3, 1, dvec3(1.75, 0.0, 0.75)),
        ];

        for data in u_v_result {
            assert_eq!(light.point_on_light(data.0, data.1), data.2);
        }
    }

    #[test]
    fn the_area_light_intensity_function() {
        let w = default_world();
        let light = AreaLight::new(
            dvec3(-0.5, -0.5, -5.0),
            dvec3(1.0, 0.0, 0.0),
            2,
            dvec3(0.0, 1.0, 0.0),
            2,
            Color::white()
        );

        let point_result = vec![
            (dvec3(0.0, 0.0, 2.0), 0.0),
            (dvec3(1.0, -1.0, 2.0), 0.25),
            (dvec3(1.5, 0.0, 2.0), 0.5),
            (dvec3(1.25, 1.25, 3.0), 0.75),
            (dvec3(0.0, 0.0, -2.0), 1.0),
        ];

        for data in point_result {
            assert_eq!(light.intensity_at(data.0, &w), data.1);
        }
    }
}
