use eframe::egui::Rgba;

use crate::{
    intersect::{Intersect, Intersection, TestIntersectionResult},
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
    // pretty much the same as the sphere intersector, minor changes
    fn test_intersection(&self, ray: &renderer::Ray, _: Rgba) -> TestIntersectionResult {
        let L = self.origin - ray.origin;
        let t_ca = L.dot(&ray.direction);

        // default colour to return if there is no intersection
        let background = 0.0;
        // i dont know if this will ever happen, probably for the best
        if t_ca < 0.0 {
            return TestIntersectionResult(
                Intersection::new(Rgba::from_gray(background), None, None),
                None,
            );
        }

        let close_approach_point = ray.at_point(t_ca); // closest approach
        let distance = (close_approach_point - self.origin).norm();

        let brightness = 1.0;

        // use a 1/r^2 dropoff
        // let brightness = t_ca * self.intensity / distance.powi(2);

        if distance < self.intensity {
            return TestIntersectionResult(
                Intersection::new(Rgba::from_gray(brightness), Some(t_ca), None),
                None,
            );
        } else {
            return TestIntersectionResult(
                Intersection::new(Rgba::from_black_alpha(0.0), None, None),
                None,
            );
        }
    }
}
