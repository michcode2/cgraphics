use eframe::egui::Rgba;
use nalgebra::{Matrix3, Vector3};

use crate::{
    intersect::{self, Intersect, Intersection},
    renderer::Ray,
};

#[derive(Clone, Copy, Debug)]
#[allow(unused)]
pub struct Plane {
    normal: Vector3<f32>,
    i: Vector3<f32>,
    j: Vector3<f32>,
    k: f32,
    origin: Vector3<f32>,
    inverse: Option<Matrix3<f32>>,
}

impl Plane {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: &Vector3<f32>, B: &Vector3<f32>, C: &Vector3<f32>) -> Plane {
        let i = B - A;
        let j = C - A;
        let normal = i.cross(&j) / (i.cross(&j)).norm();
        let k = normal.dot(A);
        let small_x = i + A;
        let small_y = j + A;
        let simul_eq = Matrix3::from_columns(&[small_x, small_y, A.clone_owned()]).transpose();
        Plane {
            normal,
            i,
            j,
            k,
            origin: A.clone_owned(),
            inverse: simul_eq.try_inverse(),
        }
    }

    pub fn in_plane_coords(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        if let Some(inv) = self.inverse {
            let return_maybe = point.transpose() * inv;
            return Some(return_maybe.transpose());
        }
        return None;
    }
}

impl Intersect for Plane {
    #[allow(non_snake_case)]
    fn test_intersection(&self, ray: &crate::renderer::Ray, _: Rgba) -> intersect::Intersection {
        // stolen from https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html

        let t_int = (self.origin - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);
        if t_int < 0.0 {
            return Intersection::new(Rgba::from_gray(0.0), None, None);
        }
        // need epsilon otherwise it gets specely
        let eps = 1e-3;
        let normal_ray = Ray::new(ray.at_point(t_int + eps), self.normal);

        //let delta = ray.direction + self.normal;

        //let reflected_ray = Ray::new(ray.at_point(t_int + eps), (delta) - ray.direction);

        return Intersection::new(Rgba::from_gray(0.5), Some(t_int), Some(normal_ray));
    }
}
