use eframe::egui::Rgba;
use nalgebra;

use crate::intersect::Intersect;

pub struct Sphere {
    pub origin: nalgebra::Vector3<f32>,
    pub radius: f32,
}

#[allow(non_snake_case)]
impl Intersect for Sphere {
    fn test_intersection(
        &self,
        ray_origin: &nalgebra::Vector3<f32>,
        ray_direction: &nalgebra::Vector3<f32>,
    ) -> eframe::egui::Rgba {
        let L = self.origin - ray_origin;
        let t_ca = L.dot(ray_direction);

        if t_ca < 0.0 {
            return Rgba::from_gray(0.0);
        }

        let CLApp = ray_origin + ray_direction * t_ca; // closest approach
        let distance = (CLApp - self.origin).norm();
        if distance < self.radius {
            return Rgba::from_rgb(CLApp.x, CLApp.y, CLApp.z);
        }
        return Rgba::from_gray(0.0);
    }
}
