use epaint::Rgba;

use crate::intersect::{Intersect, Intersection};

pub struct WorldLight {}

impl Intersect for WorldLight {
    fn test_intersection(
        &self,
        _: &crate::renderer::Ray,
        _: epaint::Rgba,
    ) -> crate::intersect::Intersection {
        return Intersection {
            colour: Rgba::from_gray(0.01),
            distance: Some(f32::MAX),
            normal: None,
        };
    }
}
