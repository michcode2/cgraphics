use eframe::egui::Rgba;

use crate::{
    intersect::{Intersect, Intersection},
    renderer,
};

#[derive(Clone, Copy)]
pub struct PointLight {
    origin: nalgebra::Vector3<f32>,
    intensity: f32,
}

impl PointLight {
    pub fn new(origin: nalgebra::Vector3<f32>, intensity: f32) -> PointLight {
        PointLight { origin, intensity }
    }
}

#[allow(non_snake_case)]
impl Intersect for PointLight {
    fn test_intersection(&self, ray: &renderer::Ray) -> crate::intersect::Intersection {
        let L = self.origin - ray.origin;
        let t_ca = L.dot(&ray.direction);

        let background = 0.0;

        if t_ca < 0.0 {
            return Intersection::new(Rgba::from_gray(background), None, None);
        }

        let close_approach_point = ray.at_point(t_ca); // closest approach
        let distance = (close_approach_point - self.origin).norm();

        let brightness = self.intensity / distance.powi(2);

        return Intersection::new(Rgba::from_gray(brightness), Some(10.0), None);
    }
}
