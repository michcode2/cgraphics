use eframe::egui::Rgba;

use crate::{
    renderer::{many_spheres, Ray},
    scene::Scene,
};

pub struct Camera {
    pub location: Ray,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    pub fn create_buffer(&self, scene: &Scene<crate::sphere::Sphere>) -> Vec<Vec<Rgba>> {
        let mut buffer: Vec<Vec<Rgba>> =
            vec![vec![Rgba::from_gray(0.0); self.height as usize]; self.width as usize];
        for x in 0..self.width {
            for y in 0..self.height {
                let x_normalised = ((-2.0 * x as f32) / self.width as f32) + 1.0;
                let y_normalised = ((2.0 * y as f32) / self.height as f32) - 1.0;

                let pixel_direction = nalgebra::Vector3::new(0.0, y_normalised, x_normalised);

                let pixel_ray = Ray::new(
                    self.location.origin,
                    pixel_direction + self.location.direction,
                );
                let color = scene.test_intersections(pixel_ray).colour;
                buffer[x as usize][y as usize] = color;
            }
        }
        buffer
    }
}
