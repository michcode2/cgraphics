use epaint::Rgba;

use crate::surfaces::Surface;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Diffuse {}
impl Surface for Diffuse {
    fn get_value(&self, other: Rgba) -> epaint::Rgba {
        return Rgba::RED;
    }
}
