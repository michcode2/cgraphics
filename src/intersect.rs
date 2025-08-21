use std::{cmp::Ordering, fmt::Debug, sync::Arc};

use eframe::egui::Rgba;

use crate::{renderer::Ray, surfaces::Surface};

#[derive(Clone, Debug)]
pub struct TestIntersectionResult(pub Intersection, pub Option<Arc<dyn Surface>>);
impl PartialOrd for TestIntersectionResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for TestIntersectionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for TestIntersectionResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Eq for TestIntersectionResult {}

pub trait Intersect: Send + Sync + Debug {
    fn test_intersection(&self, ray: &Ray, incoming_colour: Rgba) -> TestIntersectionResult;
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
