use eframe::egui::Rgba;
use nalgebra;

use crate::{
    intersect::{Intersect, Intersection},
    renderer::Ray,
};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub origin: nalgebra::Vector3<f32>,
    pub radius: f32,
    pub colour: Rgba,
}

#[allow(non_snake_case)]
impl Intersect for Sphere {
    fn test_intersection(&self, ray: &Ray) -> Intersection {
        let L = self.origin - ray.origin;
        let t_ca = L.dot(&ray.direction);

        let background = 0.0;

        if t_ca <= 0.0 {
            return Intersection::new(Rgba::from_gray(background), None, None);
        }

        let close_approach_point = ray.at_point(t_ca); // closest approach
        let distance = (close_approach_point - self.origin).norm();
        let t_surface_to_cap = (self.radius.powi(2) - distance.powi(2)).sqrt();
        let t_surface = t_ca - t_surface_to_cap;
        let surface = ray.at_point(t_surface);

        let normal_vec = surface - self.origin;
        let normal_vec = normal_vec / normal_vec.norm();

        let normal_ray = Ray::new(surface, normal_vec);

        if distance < self.radius {
            return Intersection::new(self.colour, Some(distance), Some(normal_ray));
        }
        //return Intersection::new(Rgba::from_gray(background), None, None);
        return Intersection::new(Rgba::from_rgb(0.0, 0.0, 0.0), None, None);
    }
}
