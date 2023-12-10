pub mod shapes {
    pub use shape::Shape;
    pub use sphere::Sphere;

    pub mod shape;
    pub mod sphere;
}

pub use object::Object;
pub use world::World;
pub use camera::Camera;
pub use canvas::Canvas;
pub use color::Color;

mod ray;
mod intersection;
pub mod object;
pub mod light;
pub mod material;
pub mod world;
pub mod canvas;
pub mod color;
pub mod camera;