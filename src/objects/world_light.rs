use epaint::Rgba;

use crate::{
    common_maths::maths,
    intersect::{Intersect, Intersection, TestIntersectionResult},
};

pub struct WorldLight {}

impl Intersect for WorldLight {
    fn test_intersection(
        &self,
        ray: &crate::renderer::Ray,
        _: epaint::Rgba,
    ) -> TestIntersectionResult {
        return TestIntersectionResult(
            Intersection {
                colour: Rgba::from_gray(0.1 * (1.0 + maths::normalise_vec3(&ray.direction).z)),
                distance: Some(f32::MAX),
                normal: None,
            },
            None,
        );
    }
}
