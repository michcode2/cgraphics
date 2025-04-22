use std::cmp::Ordering;

use crate::{
    intersect::{Intersect, Intersection},
    renderer::Ray,
    sphere::Sphere,
};

pub struct Scene<T: Intersect> {
    objects: Vec<T>,
}

impl<T: Intersect> Scene<T> {
    pub fn test_intersections(&self, ray: Ray) -> Intersection {
        let mut all_objects = self
            .objects
            .iter()
            .map(|obj| obj.test_intersection(&ray))
            .collect::<Vec<Intersection>>();
        all_objects.sort_by(|a, b| {
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
        all_objects[0]
    }

    pub fn new_test() -> Scene<Sphere> {
        let mut objects = vec![];

        objects.push(Sphere {
            origin: nalgebra::Vector3::new(3.0, 1.5, 3.0),
            radius: 0.75,
        });

        objects.push(Sphere {
            origin: nalgebra::Vector3::new(3.0, -3.0, 0.0),
            radius: 0.5,
        });

        Scene { objects }
    }
}
