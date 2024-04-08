mod arc;
mod circle;
mod cone;
mod line;
mod plane;
mod sphere;

pub use arc::Arc;
pub use circle::Circle;
pub use cone::Cone;
pub use line::Line;
pub use plane::Plane;
pub use sphere::Sphere;

pub trait GeometryObject {}
