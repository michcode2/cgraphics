use epaint::Rgba;

use crate::surfaces::Surface;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Diffuse {
    pub colour: Rgba,
}
impl Surface for Diffuse {
    fn get_value(&self, _: Rgba) -> epaint::Rgba {
        return self.colour;
    }
}
