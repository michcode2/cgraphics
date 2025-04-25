use eframe::egui::Rgba;

use crate::{light, renderer::Ray, sphere};

pub trait Intersect {
    fn test_intersection(&self, ray: &Ray, depth: u8) -> Intersection;
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

#[derive(Clone, Copy)]
pub enum Intersectable {
    Sphere(sphere::Sphere),
    PointLight(light::PointLight),
}

impl Intersect for Intersectable {
    fn test_intersection(&self, ray: &Ray, depth: u8) -> Intersection {
        match self {
            Intersectable::Sphere(s) => s.test_intersection(ray, depth),
            Intersectable::PointLight(l) => l.test_intersection(ray, depth),
        }
    }
}
