use eframe::egui::Rgba;

use crate::renderer::Ray;

pub trait Intersect {
    fn test_intersection(&self, ray: &Ray) -> Intersection;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub colour: Rgba,
    pub distance: Option<f32>,
}

impl Intersection {
    pub fn new(colour: Rgba, distance: Option<f32>) -> Intersection {
        Intersection { colour, distance }
    }
}
