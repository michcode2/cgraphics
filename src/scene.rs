use eframe::egui::Rgba;

use crate::{
    intersect::{Intersect, Intersectable, Intersection},
    light::{self, PointLight},
    renderer::Ray,
    sphere::Sphere,
};

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Intersectable>,
    max_depth: u8,
}

const DEPTH: u8 = 8;

impl Scene {
    pub fn test_intersections(&self, ray: Ray, current_depth: u8) -> Intersection {
        let mut all_objects = self
            .objects
            .iter()
            .map(|obj| obj.test_intersection(&ray))
            .map(|mut intersect| {
                if let Some(ray_new) = intersect.normal {
                    if current_depth < self.max_depth {
                        let result = self.test_intersections(ray_new, 0);
                        intersect.colour = result.colour + intersect.colour.multiply(0.99);
                        return intersect;
                    } else {
                        return intersect;
                    }
                } else {
                    return intersect;
                }
            })
            .collect::<Vec<Intersection>>();
        all_objects.sort();

        if let Some(_) = all_objects[0].normal {
            return all_objects[0];
        } else {
            all_objects[0].colour = all_objects[0].colour + Rgba::from_gray(-0.01);
            return all_objects[0];
        }
    }

    #[allow(dead_code)]
    pub fn curve() -> Scene {
        let mut objects = vec![];

        let num_balls = 100;
        let extent = 10.0;

        for i in 0..num_balls {
            let y = -extent + (i as f32 * extent * 2.0 / num_balls as f32);
            objects.push(Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, y, 2.0 * (y).sin() + 0.1 * y.powi(2)),
                radius: 0.1,
                colour: Rgba::from_white_alpha(1.0),
            }));
        }

        objects.push(Intersectable::PointLight(light::PointLight::new(
            nalgebra::Vector3::new(-12.0, -12.0, 20.0),
            1.0,
        )));

        objects.push(Intersectable::PointLight(light::PointLight::new(
            nalgebra::Vector3::new(-12.0, 12.0, 20.0),
            1.0,
        )));

        Scene {
            objects,
            max_depth: DEPTH,
        }
    }

    #[allow(dead_code)]
    pub fn pondering_orbs() -> Scene {
        let objects = vec![
            Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, 8.0, 8.0),
                radius: 1.0,
                colour: Rgba::from_white_alpha(1.0),
            }),
            Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, 5.0, 5.0),
                radius: 1.0,
                colour: Rgba::from_white_alpha(1.0),
            }),
            Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(0.0, 3.6, 3.9),
                radius: 0.5,
                colour: Rgba::from_rgb(1.0, 0.0, 0.0),
            }),
            Intersectable::PointLight(PointLight::new(
                nalgebra::Vector3::new(12.0, -8.0, -8.0),
                1.0,
            )),
        ];
        Scene {
            objects,
            max_depth: DEPTH,
        }
    }

    #[allow(dead_code)]
    pub fn eclipse() -> Scene {
        let objects = vec![
            Intersectable::Sphere(Sphere {
                origin: nalgebra::Vector3::new(3.0, 0.0, 0.0),
                radius: 1.0,
                colour: Rgba::from_white_alpha(1.0),
            }),
            Intersectable::PointLight(PointLight::new(nalgebra::Vector3::new(9.0, 0.0, 0.0), 1.0)),
        ];
        Scene {
            objects,
            max_depth: DEPTH,
        }
    }
}
