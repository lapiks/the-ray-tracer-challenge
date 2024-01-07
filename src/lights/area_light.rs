use glam::DVec3;
use rand::Rng;

use crate::{Color, sequence::Sequence};

use super::light::LightSource;

#[derive(Debug, Clone, PartialEq)]
pub struct AreaLight {
    corner: DVec3,
    uvec: DVec3,
    usteps: usize,
    vvec: DVec3,
    vsteps: usize,
    intensity: Color,
    samples: usize,
    positions: Vec<DVec3>,
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
        let samples = usteps * vsteps;
        let uvec = full_uvec / usteps as f64;
        let vvec = full_vvec / vsteps as f64;
        let mut positions = Vec::with_capacity(samples);
        for v in 0..vsteps {
            for u in 0..usteps {
                positions.push(corner + uvec * (u as f64 + 0.5) + vvec * (v as f64 + 0.5));
            }
        }

        Self {
            corner,
            uvec,
            usteps,
            vvec,
            vsteps,
            intensity: color,
            samples,
            positions,
        }
    }

    // returns the point in the middle of the cell at the given coordinates
    fn point_on_light(&self, u: usize, v: usize, jitter_by: &mut Sequence<f64>) -> DVec3 {
        self.corner + 
        self.uvec * (u as f64 + jitter_by.next().unwrap()) +
        self.vvec * (v as f64 + jitter_by.next().unwrap()) 
    }

    fn intensity_at_impl(&self, world_point: DVec3, world: &crate::World, jitter_by: &mut Sequence<f64>) -> f64 {
        let mut total = 0.0;
        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                total += match world.is_shadowed(
                    world_point,
                    self.point_on_light(u, v, jitter_by)
                ) {
                    true => 0.0,
                    false => 1.0,
                }
            }
        }
        total / self.samples as f64
    }
}

impl LightSource for AreaLight {
    fn positions(&self) -> &[DVec3] {
        self.positions.as_slice()
    }

    fn intensity(&self) -> crate::Color {
        self.intensity
    }

    fn intensity_at(&self, world_point: DVec3, world: &crate::World) -> f64 {
        let mut rng = rand::thread_rng();
        let mut random_values: Vec<f64> = Vec::with_capacity(self.samples);
        for _ in 0..self.samples {
            random_values.push(rng.gen());
        }
        let mut jitter_by = Sequence::new(random_values);
        self.intensity_at_impl(world_point, world, &mut jitter_by)
    }
}

#[cfg(test)]
mod tests {
    use glam::dvec3;

    use crate::{world::tests::default_world, sequence::Sequence};

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

        let mut jitter_by = Sequence::new(vec![0.5]);

        for data in u_v_result {
            assert_eq!(light.point_on_light(data.0, data.1, &mut jitter_by), data.2);
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

        let mut jitter_by = Sequence::new(vec![0.5]);

        for data in point_result {
            assert_eq!(light.intensity_at_impl(data.0, &w, &mut jitter_by), data.1);
        }
    }

    #[test]
    fn finding_a_single_point_on_a_jittered_area_light() {
        let light = AreaLight::new(
            dvec3(0.0, 0.0, 0.0),
            dvec3(2.0, 0.0, 0.0),
            4,
            dvec3(0.0, 0.0, 1.0),
            2,
            Color::white()
        );

        let u_v_result = vec![
            (0, 0, dvec3(0.15, 0.0, 0.35)),
            (1, 0, dvec3(0.65, 0.0, 0.35)),
            (0, 1, dvec3(0.15, 0.0, 0.85)),
            (2, 0, dvec3(1.15, 0.0, 0.35)),
            (3, 1, dvec3(1.65, 0.0, 0.85)),
        ];

        let mut jitter_by = Sequence::new(vec![0.3, 0.7]);

        for data in u_v_result {
            assert_eq!(light.point_on_light(data.0, data.1, &mut jitter_by), data.2);
        }
    }

    #[test]
    fn the_area_light_with_jittered_samples() {
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
            (dvec3(1.0, -1.0, 2.0), 0.5),
            (dvec3(1.5, 0.0, 2.0), 0.75),
            (dvec3(1.25, 1.25, 3.0), 0.75),
            (dvec3(0.0, 0.0, -2.0), 1.0),
        ];

        let mut jitter_by = Sequence::new(vec![0.7, 0.3, 0.9, 0.1, 0.5]);

        for data in point_result {
            assert_eq!(light.intensity_at_impl(data.0, &w, &mut jitter_by), data.1);
        }
    }
}
