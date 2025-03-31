pub use crate::interval::*;
pub use crate::ray::*;
pub use crate::vec3::*;

// pub use rand::prelude::*;

pub fn random_double() -> f64 {
    rand::random_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::random_range(min..max)
}
