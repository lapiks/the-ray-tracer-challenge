pub mod shapes {
    pub mod shape;
    pub mod sphere;
}

pub use world::World;
pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;

mod ray;
mod intersection;
mod object;
pub mod world;
pub mod canvas;
pub mod color;
pub mod camera;