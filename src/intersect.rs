use eframe::egui::Rgba;

use crate::{light, renderer::Ray, sphere};

pub trait Intersect {
    fn test_intersection(&self, ray: &Ray) -> Intersection;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub colour: Rgba,
    pub distance: Option<f32>,
    pub normal: Option<Ray>,
}

impl Intersection {
    pub fn new(colour: Rgba, distance: Option<f32>, normal: Option<Ray>) -> Intersection {
        Intersection {
            colour,
            distance,
            normal,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Intersectable {
    Sphere(sphere::Sphere),
    PointLight(light::PointLight),
}

impl Intersect for Intersectable {
    fn test_intersection(&self, ray: &Ray) -> Intersection {
        match self {
            Intersectable::Sphere(s) => s.test_intersection(ray),
            Intersectable::PointLight(l) => l.test_intersection(ray),
        }
    }
}
