use std::f32;

use epaint::Rgba;
use nalgebra::Vector3;
use rand::random_range;

use crate::{common_maths::maths, renderer::Ray, surfaces::Surface};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Diffuse {
    pub colour: Rgba,
}
impl Surface for Diffuse {
    fn get_value(&self, _: Rgba) -> epaint::Rgba {
        return self.colour;
    }

    fn request_rays(&self, normal_ray: &Ray, incoming_ray: &Ray) -> Vec<Ray> {
        let reflected = maths::reflected_ray(normal_ray, incoming_ray);
        (0..50)
            .into_iter()
            .map(|_| {
                Ray::new(
                    normal_ray.origin,
                    Diffuse::rotate_vector(&reflected.direction),
                )
            })
            .collect::<Vec<Ray>>()
    }

    fn intersections_to_colour(&self, rays: Vec<crate::intersect::TestIntersectionResult>) -> Rgba {
        let mut avg = Rgba::BLACK;
        let num_rays = rays.len();
        for r in rays.into_iter() {
            avg = avg + r.0.colour.multiply(0.5 / (num_rays) as f32);
        }
        avg = avg + self.colour.multiply(0.5);
        avg
    }
}

impl Diffuse {
    fn rotate_vector(input: &Vector3<f32>) -> Vector3<f32> {
        let length = input.norm();
        let kernel = |x: f32| (1.0 + (f32::consts::PI * x).cos()) / 2.0;
        let scale_x: f32 = kernel(random_range(0.0..1.0_f32));
        let scale_y: f32 = kernel(random_range(0.0..1.0_f32));
        let scale_z: f32 = kernel(random_range(0.0..1.0_f32));
        let mut return_val = Vector3::new(input.x * scale_x, input.y * scale_y, input.z * scale_z);
        return_val.scale_mut(return_val.norm() / length);
        return_val
    }
}
