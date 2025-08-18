use std::fmt::Debug;

use epaint::Rgba;

pub mod diffuse;
pub mod specular;

pub trait Surface: Send + Sync + Debug {
    fn get_value(&self, other: Rgba) -> Rgba;
}
