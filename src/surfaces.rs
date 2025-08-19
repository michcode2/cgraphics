use std::fmt::Debug;

use epaint::Rgba;

use crate::{intersect::TestIntersectionResult, renderer::Ray};

pub mod diffuse;
pub mod specular;

pub trait Surface: Send + Sync + Debug {
    fn get_value(&self, other: Rgba) -> Rgba;
    fn request_rays(&self, normal_ray: &Ray, incoming_ray: &Ray) -> Vec<Ray>;
    fn intersections_to_colour(&self, rays: Vec<TestIntersectionResult>) -> Rgba;
}
