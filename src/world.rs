use crate::object::Object;

pub struct World {
    objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::default(),
        }
    }
}