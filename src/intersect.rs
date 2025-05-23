use std::cmp::Ordering;

use eframe::egui::Rgba;

use crate::{light, plane, renderer::Ray, sphere, triangle};

pub trait Intersect {
    fn test_intersection(&self, ray: &Ray) -> Intersection;
}

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub colour: Rgba,
    pub distance: Option<f32>,
    pub normal: Option<Ray>,
}

impl Intersection {
    pub fn new(colour: Rgba, distance: Option<f32>, normal: Option<Ray>) -> Intersection {
        Intersection {
            colour,
            distance,
            normal,
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        if let (Some(self_dist), Some(other_dist)) = (self.distance, other.distance) {
            return self_dist == other_dist;
        } else {
            return false;
        }
    }
}

impl Eq for Intersection {}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.distance, other.distance) {
            (Some(self_dist), Some(other_dist)) => {
                if self_dist == other_dist {
                    return Some(Ordering::Equal);
                } else if self_dist < other_dist {
                    return Some(Ordering::Less);
                } else {
                    return Some(Ordering::Greater);
                }
            }
            (Some(_), _) => return Some(Ordering::Less),
            (_, Some(_)) => return Some(Ordering::Greater),
            (_, _) => return Some(Ordering::Equal),
        }
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[derive(Clone, Copy)]
pub enum Intersectable {
    Sphere(sphere::Sphere),
    PointLight(light::PointLight),
    Plane(plane::Plane),
    Triangle(triangle::Triangle),
}

impl Intersect for Intersectable {
    fn test_intersection(&self, ray: &Ray) -> Intersection {
        match self {
            Intersectable::Sphere(s) => s.test_intersection(ray),
            Intersectable::PointLight(l) => l.test_intersection(ray),
            Intersectable::Plane(p) => p.test_intersection(ray),
            Intersectable::Triangle(t) => t.test_intersection(ray),
        }
    }
}
