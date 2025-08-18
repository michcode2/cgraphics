use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{Intersect, Intersection},
    objects::plane::Plane,
};

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    inner_plane: Plane,
    colour: Rgba,
}

impl Triangle {
    #[allow(non_snake_case)]
    pub fn from_3_points(
        A: &Vector3<f32>,
        B: &Vector3<f32>,
        C: &Vector3<f32>,
        colour: Rgba,
    ) -> Triangle {
        Triangle {
            inner_plane: Plane::from_3_points(A, B, C),
            colour,
        }
    }
}

impl Intersect for Triangle {
    fn test_intersection(&self, ray: &crate::renderer::Ray, colour: Rgba) -> Intersection {
        let mut potential = self.inner_plane.test_intersection(ray, colour);
        if let Some(t) = potential.distance {
            let point = ray.at_point(t);
            let plane_coords = self.inner_plane.in_plane_coords(&point);
            if let Some(v) = plane_coords {
                let sum = v.x + v.y;
                //println!("{:?}", v);
                let bounded = |h: f32| h > 0.0 && h < 1.0;
                if bounded(sum) && bounded(v.x) && bounded(v.y) {
                    potential.colour = self.colour;
                    return potential;
                }
            }
        }
        return Intersection {
            colour: Rgba::BLACK,
            distance: None,
            normal: None,
        };
    }
}
