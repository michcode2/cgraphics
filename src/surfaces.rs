use epaint::Rgba;

pub mod diffuse;
pub mod specular;

pub trait Surface: Send + Sync {
    fn get_value(&self, other: Rgba) -> Rgba;
}
