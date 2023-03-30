mod circle;
mod line;
mod plane;
mod sphere;

pub use circle::Circle;
pub use line::Line;
pub use plane::Plane;
pub use sphere::Sphere;

pub trait GeometryObject {}
