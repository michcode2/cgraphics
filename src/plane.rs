use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{self, Intersect, Intersection},
    renderer::Ray,
};

#[derive(Clone, Copy)]
pub struct Plane {
    normal: Vector3<f32>,
    x: Vector3<f32>,
    y: Vector3<f32>,
    origin: Vector3<f32>,
}

impl Plane {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: Vector3<f32>, B: Vector3<f32>, C: Vector3<f32>) -> Plane {
        let x = B - A;
        let y = C - A;
        let normal = x.cross(&y) / (x.cross(&y)).norm();
        Plane {
            normal,
            x,
            y,
            origin: A,
        }
    }
}

impl Intersect for Plane {
    #[allow(non_snake_case)]
    fn test_intersection(&self, ray: &crate::renderer::Ray) -> intersect::Intersection {
        // the point that the ray intersects the plane will be the close approach point
        let L = self.origin - ray.origin;
        let t_ca = L.dot(&ray.direction);
        if t_ca < 0.0 {
            return Intersection::new(Rgba::from_gray(0.0), None, None);
        }
        let intersection = ray.at_point(t_ca);
        let normal_ray = Ray::new(intersection, self.normal);
        return Intersection::new(Rgba::from_gray(0.5), Some(t_ca), Some(normal_ray));
    }
}
