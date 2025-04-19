use eframe::egui::Rgba;
use nalgebra;

pub trait Intersect {
    fn test_intersection(
        &self,
        ray_origin: &nalgebra::Vector3<f32>,
        ray_direction: &nalgebra::Vector3<f32>,
    ) -> Rgba;
}
