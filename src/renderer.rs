use std::cmp::Ordering;

use eframe::egui::Rgba;
use nalgebra::{self, Vector3};

use crate::intersect::{Intersect, Intersection};
use crate::objects::sphere::Sphere;

#[allow(dead_code)]
// the little smiley face
pub fn many_spheres(x: f32, y: f32) -> Rgba {
    let first_sphere = Sphere::blank_specular_surface(nalgebra::Vector3::new(3.0, 1.5, 3.0), 0.75);

    let second_sphere =
        Sphere::blank_specular_surface(nalgebra::Vector3::new(3.0, -1.5, 3.0), 0.75);

    let mut objects = vec![first_sphere, second_sphere];

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, -3.0, 0.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, 3.0, 0.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, 2.0, -1.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, -2.0, -1.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, -1.0, -1.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, 1.0, -1.0),
        0.5,
    ));

    objects.push(Sphere::blank_specular_surface(
        nalgebra::Vector3::new(3.0, 0.0, -1.0),
        0.5,
    ));

    let direction = nalgebra::Vector3::new(0.5, y, x + 0.1);
    let origin = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    let pixel_ray = Ray::new(origin, direction);

    let mut unsorted = objects
        .into_iter()
        .map(|obj| obj.test_intersection(&pixel_ray, Rgba::from_gray(1.0)).0)
        .collect::<Vec<Intersection>>();
    unsorted.sort_by(|a, b| {
        if let Some(r_a) = a.distance {
            if let Some(r_b) = b.distance {
                if r_a < r_b {
                    return Ordering::Less;
                }
                return Ordering::Greater;
            }
            return Ordering::Less;
        }
        return Ordering::Greater;
    });
    if let Some(_) = unsorted[0].normal {
        println!("{:?}", unsorted[0].normal);
        return unsorted[0].colour;
    } else {
        return unsorted[0].colour + Rgba::from_gray(0.8);
    }
}

// this should probably go in its own file
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray {
            direction: direction / direction.norm(),
            origin,
        }
    }

    pub fn new_preserve(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { direction, origin }
    }

    pub fn at_point(&self, t: f32) -> Vector3<f32> {
        self.origin + (self.direction * t)
    }
}
