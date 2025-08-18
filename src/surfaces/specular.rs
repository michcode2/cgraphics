use epaint::Rgba;

use crate::surfaces::Surface;

#[derive(Clone, Copy, Debug)]
pub struct Specular {
    self_colour: Rgba,
}

impl Surface for Specular {
    fn get_value(&self, other: Rgba) -> epaint::Rgba {
        return self.self_colour + other;
    }
}

impl Specular {
    pub fn new() -> Specular {
        Specular {
            self_colour: Rgba::from_gray(1.0),
        }
    }
}
