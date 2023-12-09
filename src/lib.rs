pub mod shapes {
    pub mod shape;
    pub mod sphere;
}

pub use canvas::Canvas;
pub use color::Color;

mod ray;
mod intersection;
mod object;
pub mod canvas;
pub mod color;