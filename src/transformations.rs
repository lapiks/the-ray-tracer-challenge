use glam::{Mat4, Vec3, vec4};

pub fn view_transform(from: Vec3, to: Vec3, up: Vec3)-> Mat4 {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(upn);
    let true_up = left.cross(forward);
    let orientation = Mat4::from_cols(
        vec4(left.x, true_up.x, -forward.x, 0.0),
        vec4(left.y, true_up.y, - forward.y, 0.0),
        vec4(left.z, true_up.z, -forward.z, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0)
    );
    orientation * Mat4::from_translation(-from)
}

#[cfg(test)]
mod tests {
    use glam::{vec3, vec4};

    use super::*;

    const EPSILON: f32 = 0.00001;

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let t = view_transform(
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, -1.0),
            vec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, Mat4::IDENTITY);
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let t = view_transform(
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, Mat4::from_scale(vec3(-1.0, 1.0, -1.0)));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let t = view_transform(
            vec3(0.0, 0.0, 8.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0)
        );
        assert_eq!(t, Mat4::from_translation(vec3(0.0, 0.0, -8.0)));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let t = view_transform(
            vec3(1.0, 3.0, 2.0),
            vec3(4.0, -2.0, 8.0),
            vec3(1.0, 1.0, 0.0)
        );
        assert!(t.col(0).abs_diff_eq(vec4(-0.50709, 0.76772, -0.35857, 0.0), EPSILON));
        assert!(t.col(1).abs_diff_eq(vec4(0.50709, 0.60609, 0.59761, 0.0), EPSILON));
        assert!(t.col(2).abs_diff_eq(vec4(0.67612, 0.12122, -0.71714, 0.0), EPSILON));
        assert!(t.col(3).abs_diff_eq(vec4(-2.36643, -2.82843, 0.0, 1.0), EPSILON));
    }
}