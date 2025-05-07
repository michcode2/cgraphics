use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{
    intersect::{Intersect, Intersectable, Intersection},
    light::{self, PointLight},
    plane,
    renderer::Ray,
    sphere::Sphere,
    triangle::{self, Triangle},
};

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Intersectable>,
    max_depth: u8,
}

// max number of bounces
const DEPTH: u8 = 4;

impl Scene {
    pub fn test_intersections(&self, ray: Ray, current_depth: u8) -> Intersection {
        // go over each object in the scene
        //println!("{}", current_depth);
        let mut all_objects = self
            .objects
            .iter()
            // go over each object in the scene and find the intersections
            .map(|obj| obj.test_intersection(&ray))
            .min()
            .unwrap();

        let mut intersect = all_objects; // if the normal is a value, it implies that something has been hit
        if let Some(ray_new) = intersect.normal {
            // do another bounce if theres still bounces avaliable
            if current_depth < self.max_depth {
                let result = self.test_intersections(ray_new, current_depth + 1);
                // adjust the colour a little bit
                intersect.colour = result.colour + intersect.colour.multiply(0.2);
            }
        };
        // order all the objects by how far they are and return the closest one

        let index = 0;

        let mut return_value = intersect;

        if let Some(_) = return_value.normal {
            return return_value;
        } else {
            return_value.colour = return_value.colour + Rgba::from_gray(-0.01);
            return return_value;
        }
    }

    pub fn test_intersections_vec(&self, ray: Ray) -> Intersection {
        let mut to_process = vec![(ray, 0)];
        let mut colours = vec![];

        let mut final_intersection = Intersection {
            colour: Rgba::BLACK,
            distance: None,
            normal: None,
        };
        while let Some((this_ray, depth)) = to_process.pop() {
            if depth > self.max_depth {
                break;
            }
            let mut intersections = self
                .objects
                .iter()
                .map(|obj| obj.test_intersection(&this_ray))
                .collect::<Vec<Intersection>>();
            intersections.sort();

            if depth == 0 {
                final_intersection = intersections[0];
            } else {
                colours.push(intersections[0].colour)
            }
            if let Some(reflection) = intersections[0].normal {
                to_process.push((reflection, depth + 1));
            } else {
                break;
            }
        }

        colours.reverse();

        colours
            .iter()
            .for_each(|&c| final_intersection.colour = final_intersection.colour + c.multiply(1.0));

        return final_intersection;
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
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(1.0, 0.0, 0.0);
        let c = Vector3::new(0.1, 1.0, -0.1);

        let plane = plane::Plane::from_3_points(&a, &b, &c);

        let d = Vector3::new(-6.0, 1.0, 1.0);
        let e = Vector3::new(-5.0, 3.0, 2.0);
        let f = Vector3::new(-6.0, 2.5, 3.0);

        let triangle = triangle::Triangle::from_3_points(&d, &e, &f, Rgba::from_rgb(0.0, 1.0, 0.0));

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
                nalgebra::Vector3::new(12.0, 0.0, 10.0),
                1.0,
            )),
            Intersectable::Plane(plane),
            Intersectable::Triangle(triangle),
            Intersectable::Sphere(Sphere {
                origin: d,
                radius: 0.1,
                colour: Rgba::from_rgb(1.0, 0.0, 0.0),
            }),
            Intersectable::Sphere(Sphere {
                origin: e,
                radius: 0.1,
                colour: Rgba::from_rgb(1.0, 0.0, 0.0),
            }),
            Intersectable::Sphere(Sphere {
                origin: f,
                radius: 0.1,
                colour: Rgba::from_rgb(1.0, 0.0, 0.0),
            }),
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

    pub fn from_csv(path: String) -> Scene {
        let mut objects: Vec<Intersectable> = vec![];

        let make_sphere = |mut input: Vec<&str>| {
            input.remove(0);
            let numbers = input
                .into_iter()
                .map(|val: &str| val.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();

            let object = Sphere {
                origin: Vector3::new(numbers[0], numbers[1], numbers[2]),
                radius: numbers[3],
                colour: Rgba::from_rgb(numbers[4], numbers[5], numbers[6]),
            };
            return Intersectable::Sphere(object);
        };

        let make_light = |mut input: Vec<&str>| {
            input.remove(0);
            let numbers = input
                .into_iter()
                .map(|val: &str| val.parse::<f32>().expect(&format!("yikes! {}", val)))
                .collect::<Vec<f32>>();

            let origin = Vector3::new(numbers[0], numbers[1], numbers[2]);
            let object = PointLight::new(origin, numbers[3]);
            return Intersectable::PointLight(object);
        };

        let make_triangle = |mut input: Vec<&str>| {
            input.remove(0);
            let numbers = input
                .into_iter()
                .map(|val: &str| val.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();

            let a = Vector3::new(numbers[0], numbers[1], numbers[2]);
            let b = Vector3::new(numbers[3], numbers[4], numbers[5]);
            let c = Vector3::new(numbers[6], numbers[7], numbers[8]);
            let colour = Rgba::from_rgb(numbers[9], numbers[10], numbers[11]);
            let tri = Triangle::from_3_points(&a, &b, &c, colour);
            return Intersectable::Triangle(tri);
        };

        let lines = std::fs::read_to_string(path).unwrap();
        for line in lines.lines() {
            let data = line.split(",").collect::<Vec<&str>>();
            match data[0] {
                "s" => objects.push(make_sphere(data)),
                "l" => objects.push(make_light(data)),
                "t" => objects.push(make_triangle(data)),
                _ => println!("bad input: {:?}", line),
            }
        }
        Scene {
            objects,
            max_depth: DEPTH,
        }
    }
}
