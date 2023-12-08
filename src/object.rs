use glam::Mat4;

use crate::shapes::shape::ShapeRef;

pub struct Object<'a> {
    shape: ShapeRef<'a>,
    transform: Mat4,
}

impl<'a> Object<'a> {
    pub fn new(shape: ShapeRef<'a>) -> Self {
        Self {
            shape,
            transform: Mat4::IDENTITY
        }
    }
}