use glam::{DMat4, DVec3, dvec4};

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

    use super::*;

    const EPSILON: f64 = 0.00001;

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