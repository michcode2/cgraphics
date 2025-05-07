use eframe::egui::Rgba;
use nalgebra::{Matrix3, Vector3};

use crate::{
    intersect::{self, Intersect, Intersection},
    renderer::Ray,
};

#[derive(Clone, Copy, Debug)]
pub struct Plane {
    normal: Vector3<f32>,
    x: Vector3<f32>,
    y: Vector3<f32>,
    k: f32,
    origin: Vector3<f32>,
    inverse: Option<Matrix3<f32>>,
}

impl Plane {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: &Vector3<f32>, B: &Vector3<f32>, C: &Vector3<f32>) -> Plane {
        let x = B - A;
        let y = C - A;
        let normal = x.cross(&y) / (x.cross(&y)).norm();
        let k = normal.dot(A);
        let small_x = x + A;
        let small_y = y + A;
        let simul_eq = Matrix3::from_columns(&[small_x, small_y, A.clone_owned()]).transpose();
        Plane {
            normal,
            x,
            y,
            k,
            origin: A.clone_owned(),
            inverse: simul_eq.try_inverse(),
        }
    }

    pub fn from_3_points_no_inverse(A: &Vector3<f32>, B: &Vector3<f32>, C: &Vector3<f32>) -> Plane {
        let x = B - A;
        let y = C - A;
        let normal = x.cross(&y) / (x.cross(&y)).norm();
        let k = normal.dot(A);
        Plane {
            normal,
            x,
            y,
            k,
            origin: A.clone_owned(),
            inverse: None,
        }
    }

    fn point_on_plane(&self, point: &Vector3<f32>) -> bool {
        let new_plane = Plane::from_3_points_no_inverse(
            point,
            &(self.x + self.origin),
            &(self.y + self.origin),
        );

        let k_close = (new_plane.k - self.k).abs() < 1e-2;
        let n_close = (new_plane.normal - self.normal).sum().abs() < 1e-2;

        k_close && n_close
    }

    pub fn in_plane_coords(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        // if !self.point_on_plane(point) {
        //     return None;
        // }
        
        
        
        if let Some(inv) = self.inverse {
            let return_maybe = point.transpose() * inv;
            return Some(return_maybe.transpose());
        }
        return None;
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
        // need epsilon otherwise it gets specely
        let eps = 1e-3;
        let normal_ray = Ray::new(ray.at_point(t_int + eps), self.normal);
        return Intersection::new(Rgba::from_gray(0.5), Some(t_int), Some(normal_ray));
    }
}
