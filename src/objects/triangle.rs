use std::sync::Arc;

use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{Intersect, Intersection, TestIntersectionResult},
    objects::plane::Plane,
    surfaces::Surface,
};

#[derive(Clone, Debug)]
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

    #[allow(non_snake_case)]
    pub fn from_3_points_and_surface(
        A: &Vector3<f32>,
        B: &Vector3<f32>,
        C: &Vector3<f32>,
        surface: Arc<dyn Surface>,
    ) -> Triangle {
        Triangle {
            inner_plane: Plane::from_3_points_and_surface(A, B, C, surface),
            colour: Rgba::BLACK,
        }
    }
}

impl Intersect for Triangle {
    fn test_intersection(
        &self,
        ray: &crate::renderer::Ray,
        colour: Rgba,
    ) -> TestIntersectionResult {
        let TestIntersectionResult(mut potential, surface) =
            self.inner_plane.test_intersection(ray, colour);
        if let Some(t) = potential.distance {
            let point = ray.at_point(t);
            let plane_coords = self.inner_plane.in_plane_coords(&point);
            if let Some(v) = plane_coords {
                let sum = v.x + v.y;
                //println!("{:?}", v);
                let bounded = |h: f32| h > 0.0 && h < 1.0;
                if bounded(sum) && bounded(v.x) && bounded(v.y) {
                    potential.colour = self.colour;
                    return TestIntersectionResult(potential, surface);
                }
            }
        }
        return TestIntersectionResult(
            Intersection {
                colour: Rgba::BLACK,
                distance: None,
                normal: None,
            },
            None,
        );
    }
}
