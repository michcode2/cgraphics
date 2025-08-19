use nalgebra::Vector3;

use crate::renderer::Ray;

#[allow(dead_code)]
pub fn normalise_vec3(input: &Vector3<f32>) -> Vector3<f32> {
    input / input.norm()
}

#[allow(dead_code)]
pub fn reflected_ray(normal: &Ray, incoming: &Ray) -> Ray {
    let reflected_direction: Vector3<f32> = incoming.direction + normal.direction.scale(2.0);
    let reflected_ray = Ray::new_preserve(normal.origin, reflected_direction);
    reflected_ray
}
