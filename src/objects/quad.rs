use std::sync::Arc;

use epaint::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{Intersect, Intersection, TestIntersectionResult},
    objects::plane::Plane,
    surfaces::{diffuse::Diffuse, Surface},
};

#[derive(Debug)]
pub struct Quad {
    inner_plane: Plane,
    surface: Arc<dyn Surface>,
}

impl Quad {
    #[allow(non_snake_case)]
    pub fn from_3_points(A: &Vector3<f32>, B: &Vector3<f32>, C: &Vector3<f32>) -> Quad {
        Quad {
            inner_plane: Plane::from_3_points(A, B, C),
            surface: Arc::new(Diffuse {
                colour: Rgba::from_gray(0.5),
                samples: 3,
            }),
        }
    }

    #[allow(non_snake_case)]
    pub fn from_3_points_and_surface(
        A: &Vector3<f32>,
        B: &Vector3<f32>,
        C: &Vector3<f32>,
        surface: Arc<dyn Surface>,
    ) -> Quad {
        Quad {
            inner_plane: Plane::from_3_points(A, B, C),
            surface,
        }
    }
}

impl Intersect for Quad {
    fn test_intersection(
        &self,
        ray: &crate::renderer::Ray,
        colour: Rgba,
    ) -> TestIntersectionResult {
        let TestIntersectionResult(potential, _) = self.inner_plane.test_intersection(ray, colour);
        if let Some(t) = potential.distance {
            let point = ray.at_point(t);
            let plane_coords = self.inner_plane.in_plane_coords(&point);
            if let Some(v) = plane_coords {
                let bounded = |h: f32| h > 0.0 && h < 1.0;
                if bounded(v.x) && bounded(v.y) {
                    return TestIntersectionResult(potential, Some(self.surface.clone()));
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
