pub mod specular;

pub trait Surface {
    fn get_value(&self, other: Rgba) -> Rgba;
}
