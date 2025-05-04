use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{Intersect, Intersection},
    plane::Plane,
};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    inner_plane: Plane,
}

impl Triangle {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: &Vector3<f32>, B: &Vector3<f32>, C: &Vector3<f32>) -> Triangle {
        Triangle {
            inner_plane: Plane::from_3_points(A, B, C),
        }
    }
}

impl Intersect for Triangle {
    fn test_intersection(&self, ray: &crate::renderer::Ray) -> Intersection {
        let mut potential = self.inner_plane.test_intersection(ray);
        if let Some(t) = potential.distance {
            let point = ray.at_point(t);
            let plane_coords = self.inner_plane.in_plane_coords(&point);
            if let Some(v) = plane_coords {
                let sum = v.x + v.y;
                //println!("{:?}", v);
                let bounded = |h: f32| h > 0.0 && h < 1.0;
                if bounded(sum) && bounded(v.x) && bounded(v.y) {
                    potential.colour = Rgba::from_rgb(1.0, 0.0, 0.0);
                    return potential;
                }
            }
        }
        return Intersection {
            colour: Rgba::from_gray(0.5),
            distance: None,
            normal: None,
        };
    }
}
