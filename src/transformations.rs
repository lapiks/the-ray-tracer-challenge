use glam::{DMat4, DVec3, dvec4};

pub trait Transformable {
    fn apply_transform(&mut self, transform: Transform);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub matrix: DMat4,
    pub inverse_matrix: DMat4,
}

impl Default for Transform {
    fn default() -> Self {
        Self { 
            matrix: DMat4::IDENTITY, 
            inverse_matrix: DMat4::IDENTITY
        }
    }
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_matrix(matrix: DMat4) -> Self {
        Self {
            matrix,
            inverse_matrix: matrix.inverse()
        }
    }

    pub fn apply(self, other: Transform) -> Self {
        Transform::from_matrix(other.matrix * self.matrix)
    }

    pub fn with_translation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.matrix = DMat4::from_translation(DVec3::new(x, y, z)) * self.matrix;
        self.inverse_matrix = self.matrix.inverse();
        self
    }

    pub fn with_scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.matrix = DMat4::from_scale(DVec3::new(x, y, z)) * self.matrix;
        self.inverse_matrix = self.matrix.inverse();
        self
    }

    pub fn with_rotation_x(mut self, angle: f64) -> Self {
        self.matrix = DMat4::from_rotation_x(angle) * self.matrix;
        self.inverse_matrix = self.matrix.inverse();
        self
    }

    pub fn with_rotation_y(mut self, angle: f64) -> Self {
        self.matrix = DMat4::from_rotation_y(angle) * self.matrix;
        self.inverse_matrix = self.matrix.inverse();
        self
    }

    pub fn with_rotation_z(mut self, angle: f64) -> Self {
        self.matrix = DMat4::from_rotation_z(angle) * self.matrix;
        self.inverse_matrix = self.matrix.inverse();
        self
    }

    pub fn translation(&self) -> DVec3 {
        let s_r_t = self.matrix.to_scale_rotation_translation();
        s_r_t.2
    }

    pub fn scale(&self) -> DVec3 {
        let s_r_t = self.matrix.to_scale_rotation_translation();
        s_r_t.0
    }
}

pub struct TransformBuilder<T> {
    transform: Transform,
    object: T,
}

impl<T> TransformBuilder<T> 
where T: Transformable {
    pub fn new(transform: Transform, object: T) -> Self {
        Self {
            transform,
            object,
        }
    }
    pub fn with_translation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transform = self.transform.with_translation(x, y, z);
        self
    }

    pub fn with_scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.transform = self.transform.with_scale(x, y, z);
        self
    }

    pub fn with_rotation_x(mut self, angle: f64) -> Self {
        self.transform = self.transform.with_rotation_x(angle);
        self
    }

    pub fn with_rotation_y(mut self, angle: f64) -> Self {
        self.transform = self.transform.with_rotation_y(angle);
        self
    }

    pub fn with_rotation_z(mut self, angle: f64) -> Self {
        self.transform = self.transform.with_rotation_z(angle);
        self
    }

    pub fn transform(mut self) -> T {
        self.object.apply_transform(self.transform);
        self.object
    }
}

pub fn view_transform(from: DVec3, to: DVec3, up: DVec3)-> DMat4 {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);
    let orientation = DMat4::from_cols(
        dvec4(left.x, true_up.x, -forward.x, 0.0),
        dvec4(left.y, true_up.y, - forward.y, 0.0),
        dvec4(left.z, true_up.z, -forward.z, 0.0),
        dvec4(0.0, 0.0, 0.0, 1.0)
    );
    orientation * DMat4::from_translation(-from)
}

#[cfg(test)]
mod tests {
    use glam::{dvec3, dvec4};

    use crate::{Object, shapes::{Sphere, Shape}};

    use super::*;

    const EPSILON: f64 = 0.00001;

    #[test]
    fn applying_transformations_with_the_transform_builder() {
        let o = Object::new(Shape::Sphere(Sphere::default()))
        .with_scale(0.5, 0.5, 0.5)
        .with_translation(2.0, 2.0, 2.0)
        .transform();

        assert_eq!(o.transform().scale(), dvec3(0.5, 0.5, 0.5));
        assert_eq!(o.transform().translation(), dvec3(2.0, 2.0, 2.0));
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let t = view_transform(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, -1.0),
            dvec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, DMat4::IDENTITY);
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let t = view_transform(
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 0.0, 1.0),
            dvec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, DMat4::from_scale(dvec3(-1.0, 1.0, -1.0)));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let t = view_transform(
            dvec3(0.0, 0.0, 8.0),
            dvec3(0.0, 0.0, 0.0),
            dvec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, DMat4::from_translation(dvec3(0.0, 0.0, -8.0)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let t = view_transform(
            dvec3(1.0, 3.0, 2.0),
            dvec3(4.0, -2.0, 8.0),
            dvec3(1.0, 1.0, 0.0)
        );
        assert!(t.col(0).abs_diff_eq(dvec4(-0.50709, 0.76772, -0.35857, 0.0), EPSILON));
        assert!(t.col(1).abs_diff_eq(dvec4(0.50709, 0.60609, 0.59761, 0.0), EPSILON));
        assert!(t.col(2).abs_diff_eq(dvec4(0.67612, 0.12122, -0.71714, 0.0), EPSILON));
        assert!(t.col(3).abs_diff_eq(dvec4(-2.36643, -2.82843, 0.0, 1.0), EPSILON));
    }
}