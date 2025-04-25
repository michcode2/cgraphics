use std::cmp::Ordering;

use crate::{
    intersect::{Intersect, Intersectable, Intersection},
    light::{self, PointLight},
    renderer::Ray,
    sphere::Sphere,
};

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Intersectable>,
}

impl Scene {
    pub fn test_intersections(&self, ray: Ray) -> Intersection {
        let mut all_objects = self
            .objects
            .iter()
            .map(|obj| obj.test_intersection(&ray, 2))
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

    #[allow(dead_code)]
    pub fn new_test() -> Scene {
        let mut objects = vec![];

        let num_balls = 500;
        let extent = 10.0;

        for i in 0..num_balls {
            let y = -extent + (i as f32 * extent * 2.0 / num_balls as f32);
            objects.push(Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, y, 2.0 * (y).sin() + 0.1 * y.powi(2)),
                radius: 0.05,
            }));
        }

        objects.push(Intersectable::PointLight(light::PointLight::new(
            nalgebra::Vector3::new(0.0, 0.0, 0.0),
            1.0,
        )));

        Scene { objects }
    }

    pub fn pondering_orbs() -> Scene {
        let objects = vec![
            Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, 8.0, 8.0),
                radius: 1.0,
            }),
            Intersectable::PointLight(PointLight::new(
                nalgebra::Vector3::new(3.0, -8.0, -8.0),
                1.0,
            )),
        ];
        Scene { objects }
    }
}
