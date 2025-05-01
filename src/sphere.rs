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
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-Sphere-intersection.html
    fn test_intersection(&self, ray: &Ray) -> Intersection {
        let L = self.origin - ray.origin;
        let t_ca = L.dot(&ray.direction);

        // return black if the sphere is behind the camera
        let background = 0.0;
        if t_ca <= 0.0 {
            return Intersection::new(Rgba::from_gray(background), None, None);
        }

        let close_approach_point = ray.at_point(t_ca); // closest approach
        let distance = (close_approach_point - self.origin).norm();

        let t_surface_to_cap = (self.radius.powi(2) - distance.powi(2)).sqrt(); // how far it is to reach close approach from the surface for the ray
        let t_surface = t_ca - t_surface_to_cap;
        let surface = ray.at_point(t_surface); // find the location of the intersection in world coordinates

        // the normal is in the same direction as the radius to the surface
        let normal_vec = surface - self.origin;
        let normal_vec = normal_vec / normal_vec.norm();

        let normal_ray = Ray::new(surface, normal_vec);

        // return the colour of the sphere if the ray intersects, else the background colour
        if distance < self.radius {
            return Intersection::new(self.colour, Some(distance), Some(normal_ray));
        }
        return Intersection::new(Rgba::from_gray(background), None, None);
    }
}
