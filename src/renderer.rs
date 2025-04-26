use std::cmp::Ordering;

use eframe::egui::Rgba;
use nalgebra::{self, Vector3};

use crate::intersect::{Intersect, Intersection};
use crate::sphere::Sphere;

#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn single_sphere(x: f32, y: f32) -> Rgba {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-Sphere-intersection.html
    let C = nalgebra::Vector3::new(2.0, 0.0, 0.0); // center of Sphere
    let r = 0.5;
    let O = nalgebra::Vector3::new(0.0, 0.0, 0.0); // ray origin
    let D = nalgebra::Vector3::new(1.0, y, x);
    let L = C - O;
    let t_ca = L.dot(&D);

    if t_ca < 0.0 {
        return Rgba::from_gray(1.0);
    }

    let CLApp = O + D * t_ca; // closest approach
    let distance = (CLApp - C).norm();
    if distance < r {
        return Rgba::from_rgb(CLApp.x, CLApp.y, CLApp.z);
    }
    return Rgba::from_gray(1.0);
}

#[allow(dead_code)]
pub fn coordinates(x: f32, y: f32) -> Rgba {
    return Rgba::from_rgb(x, y, 0.0);
}

#[allow(dead_code)]
pub fn many_spheres(x: f32, y: f32) -> Rgba {
    let first_sphere = Sphere {
        origin: nalgebra::Vector3::new(3.0, 1.5, 3.0),
        radius: 0.75,
    };

    let second_sphere = Sphere {
        origin: nalgebra::Vector3::new(3.0, -1.5, 3.0),
        radius: 0.75,
    };

    let mut objects = vec![first_sphere, second_sphere];

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, -3.0, 0.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, 3.0, 0.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, 2.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, -2.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, -1.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, 1.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(3.0, 0.0, -1.0),
        radius: 0.5,
    });

    let direction = nalgebra::Vector3::new(0.5, y, x + 0.1);
    let origin = nalgebra::Vector3::new(0.0, 0.0, 0.0);
    let pixel_ray = Ray::new(origin, direction);

    let mut unsorted = objects
        .into_iter()
        .map(|obj| obj.test_intersection(&pixel_ray))
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
    unsorted[0].colour
}

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
