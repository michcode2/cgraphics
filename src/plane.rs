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
    k: f32,
    origin: Vector3<f32>,
}

impl Plane {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: Vector3<f32>, B: Vector3<f32>, C: Vector3<f32>) -> Plane {
        let x = B - A;
        let y = C - A;
        let normal = x.cross(&y) / (x.cross(&y)).norm();
        let k = normal.dot(&A);
        Plane {
            normal,
            x,
            y,
            k,
            origin: A,
        }
    }
}

impl Intersect for Plane {
    #[allow(non_snake_case)]
    fn test_intersection(&self, ray: &crate::renderer::Ray) -> intersect::Intersection {
        // stolen from https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html

        let t_int = (self.origin - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);
        if t_int < 0.0 {
            return Intersection::new(Rgba::from_gray(0.0), None, None);
        }
        let normal_ray = Ray::new(ray.at_point(t_int), self.normal);
        return Intersection::new(Rgba::from_gray(0.5), Some(t_int), Some(normal_ray));
    }
}
