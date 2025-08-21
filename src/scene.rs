use std::{fs, sync::Arc};

use eframe::egui::Rgba;
use nalgebra::Vector3;
use serde_json::Value;

use crate::{
    intersect::{Intersect, Intersection, TestIntersectionResult},
    objects::{quad::Quad, sphere::Sphere, triangle::Triangle, *},
    renderer::Ray,
    surfaces::{
        diffuse::{self, Diffuse},
        specular::Specular,
        Surface,
    },
};

#[derive(Clone, Debug)]
pub struct Scene {
    objects: Vec<Arc<dyn Intersect>>,
    max_depth: u8,
}

// max number of bounces
const DEPTH: u8 = 4;

impl Scene {
    #[allow(dead_code)]
    pub fn test_intersections(&self, ray: Ray, current_depth: u8) -> TestIntersectionResult {
        // go over each object in the scene
        //println!("{}", current_depth);
        let all_objects = self
            .objects
            .iter()
            // go over each object in the scene and find the intersections
            .map(|obj| obj.test_intersection(&ray, Rgba::from_gray(1.0)))
            .collect::<Vec<TestIntersectionResult>>();

        let mut intersect = all_objects.clone().into_iter().min().unwrap(); // if the normal is a value, it implies that something has been hit

        if let Some(normal_ray) = intersect.0.normal {
            if current_depth < self.max_depth {
                //aiming to improve this so that the surfaces can give rays to render and recieve the information

                // do another bounce if theres still bounces avaliable
                let new_rays = intersect
                    .clone()
                    .1
                    .unwrap()
                    .request_rays(&normal_ray, &ray)
                    .into_iter()
                    .map(|r| self.test_intersections(r, current_depth + 1))
                    .collect::<Vec<TestIntersectionResult>>();

                intersect.0.colour = intersect
                    .1
                    .clone()
                    .unwrap()
                    .intersections_to_colour(new_rays);
            }
        };
        // order all the objects by how far they are and return the closest one

        return intersect;
    }

    #[allow(dead_code)]
    pub fn test_intersections_vec(&self, ray: Ray) -> Intersection {
        // this is being kept around because if i get smart, maybe it will come in handy to give me more control
        let mut to_process = vec![(ray, 0)];
        let mut colours = vec![Rgba::BLACK];

        let mut all_intersections = vec![];

        while let Some((this_ray, depth)) = to_process.pop() {
            if depth > self.max_depth {
                break;
            }
            let mut intersections = self
                .objects
                .iter()
                .map(|obj| obj.test_intersection(&this_ray, Rgba::from_gray(0.0)).0)
                .collect::<Vec<Intersection>>();
            intersections.sort();

            let a = intersections
                .iter()
                .filter(|i| i.distance.is_some())
                .collect::<Vec<&Intersection>>();
            if a.len() > 1 {
                println!("{:?}", intersections);
            }

            all_intersections.push(intersections[0]);

            if let Some(normal) = intersections[0].normal {
                let reflected_direction: Vector3<f32> = ray.direction + normal.direction.scale(2.0);
                let reflected_ray = Ray::new_preserve(normal.origin, reflected_direction);
                to_process.push((reflected_ray, depth + 1));
            } else {
                break;
            }
        }

        colours.reverse();
        //final_intersection.colour = colours[0];

        if colours.len() > 1 {
            println!("{:?}", colours);
        }

        //colours
        //    .iter()
        //    .for_each(|&c| final_intersection.colour = final_intersection.colour + c.multiply(1.0));

        return all_intersections[all_intersections.len() - 1];
    }

    #[allow(dead_code)]
    pub fn curve() -> Scene {
        let mut objects: Vec<Arc<dyn Intersect>> = vec![];

        let num_balls = 100;
        let extent = 10.0;

        for i in 0..num_balls {
            let y = -extent + (i as f32 * extent * 2.0 / num_balls as f32);
            objects.push(Arc::new(sphere::Sphere::blank_specular_surface(
                nalgebra::Vector3::new(3.0, y, 2.0 * (y).sin() + 0.1 * y.powi(2)),
                0.1,
            )));
        }

        objects.push(Arc::new(light::PointLight::new(
            nalgebra::Vector3::new(-12.0, -12.0, 20.0),
            1.0,
        )));

        objects.push(Arc::new(light::PointLight::new(
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

        let plane = plane::Plane::from_3_points_and_surface(&a, &b, &c, Arc::new(Specular::new()));

        let d = Vector3::new(-7.0, 4.0, 1.0);
        let e = Vector3::new(-0.0, 4.0, 1.0);
        let f = Vector3::new(-7.0, 4.0, 3.0);

        let triangle = triangle::Triangle::from_3_points(&e, &d, &f, Rgba::from_rgb(0.0, 1.0, 0.0));

        let h = Vector3::new(5.0, -4.0, 1.0);
        let i = Vector3::new(5.0, -2.0, 1.0);
        let j = Vector3::new(5.0, -4.0, 9.0);

        let quad = quad::Quad::from_3_points(&h, &i, &j);

        let objects: Vec<Arc<dyn Intersect>> = vec![
            Arc::new(sphere::Sphere::blank_specular_surface(
                nalgebra::Vector3::new(3.0, 8.0, 8.0),
                1.0,
            )),
            Arc::new(sphere::Sphere::blank_specular_surface(
                nalgebra::Vector3::new(3.0, 5.0, 5.0),
                1.0,
            )),
            Arc::new(sphere::Sphere::with_shader(
                nalgebra::Vector3::new(0.0, 3.6, 3.9),
                1.0,
                Arc::new(Diffuse {
                    colour: Rgba::GREEN,
                    samples: 30,
                }),
            )),
            Arc::new(light::PointLight::new(
                nalgebra::Vector3::new(12.0, 0.0, 10.0),
                1.0,
            )),
            Arc::new(plane),
            Arc::new(triangle),
            Arc::new(sphere::Sphere::blank_specular_surface(d, 0.1)),
            Arc::new(sphere::Sphere::blank_specular_surface(e, 0.1)),
            Arc::new(sphere::Sphere::blank_specular_surface(f, 0.1)),
            Arc::new(world_light::WorldLight {}),
            Arc::new(quad),
        ];
        Scene {
            objects,
            max_depth: DEPTH,
        }
    }

    #[allow(dead_code)]
    pub fn eclipse() -> Scene {
        let objects: Vec<Arc<dyn Intersect>> = vec![
            Arc::new(sphere::Sphere::with_shader(
                nalgebra::Vector3::new(1.0, 0.0, 0.0),
                0.8,
                Arc::new(diffuse::Diffuse {
                    colour: Rgba::BLUE,
                    samples: 1,
                }),
            )),
            Arc::new(light::PointLight::new(
                nalgebra::Vector3::new(3.0, 0.0, 0.0),
                1.0,
            )),
            Arc::new(world_light::WorldLight {}),
        ];
        Scene {
            objects,
            max_depth: DEPTH,
        }
    }

    #[allow(dead_code)]
    pub fn from_csv(path: String) -> Scene {
        let mut objects: Vec<Arc<dyn Intersect>> = vec![];

        let make_sphere = |mut input: Vec<&str>| {
            input.remove(0);
            let numbers = input
                .into_iter()
                .map(|val: &str| val.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();

            let object = sphere::Sphere::blank_specular_surface(
                Vector3::new(numbers[0], numbers[1], numbers[2]),
                numbers[3],
            );
            return Arc::new(object);
        };

        let make_light = |mut input: Vec<&str>| {
            input.remove(0);
            let numbers = input
                .into_iter()
                .map(|val: &str| val.parse::<f32>().expect(&format!("yikes! {}", val)))
                .collect::<Vec<f32>>();

            let origin = Vector3::new(numbers[0], numbers[1], numbers[2]);
            let object = light::PointLight::new(origin, numbers[3]);
            return Arc::new(object);
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
            let tri = triangle::Triangle::from_3_points(&a, &b, &c, colour);
            return Arc::new(tri);
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

    pub fn from_json(file_name: &str) -> Scene {
        let file_contents = String::from_utf8(fs::read(file_name).unwrap()).unwrap();
        let contents_objects: Value = serde_json::from_str(&file_contents).unwrap();
        let a = contents_objects["items"].as_array().unwrap();
        let mut objects: Vec<Arc<dyn Intersect>> = vec![];
        for item in a {
            match item["kind"].as_str() {
                Some("sphere") => {
                    objects.push(Arc::new(Scene::parse_sphere(&item)));
                }
                Some("triangle") => {
                    objects.push(Arc::new(Scene::parse_triangle(&item)));
                }
                Some("quad") => {
                    objects.push(Arc::new(Scene::parse_quad(&item)));
                }
                None => {
                    panic!("kind not specified")
                }
                Some(_) => {
                    panic!("invalid kind of object")
                }
            }
        }
        Scene {
            objects,
            max_depth: 1,
        }
    }

    fn parse_diffuse(data: &Value) -> Arc<dyn Surface> {
        let colour = Rgba::from_rgb(
            data["colour"][0].as_f64().unwrap() as f32,
            data["colour"][1].as_f64().unwrap() as f32,
            data["colour"][2].as_f64().unwrap() as f32,
        );
        Arc::new(Diffuse::new(
            colour,
            data["samples"].as_u64().unwrap() as usize,
        ))
    }

    fn parse_specular(data: &Value) -> Arc<dyn Surface> {
        let colour = Rgba::from_rgb(
            data["colour"][0].as_f64().unwrap() as f32,
            data["colour"][1].as_f64().unwrap() as f32,
            data["colour"][2].as_f64().unwrap() as f32,
        );
        Arc::new(Specular::with_colour(colour))
    }

    fn parse_surface(data: &Value) -> Arc<dyn Surface> {
        match data["type"].as_str() {
            Some("specular") => Scene::parse_specular(&data),
            Some("diffuse") => Scene::parse_diffuse(&data),
            None => Arc::new(Specular::new()),
            Some(_) => panic!("invalid surface type"),
        }
    }

    fn parse_sphere(data: &Value) -> Sphere {
        let origin = Scene::parse_vec3(&data["origin"]);
        let radius = data["radius"].as_f64().unwrap() as f32;
        let surface = Scene::parse_surface(&data["surface"]);
        return Sphere::with_shader(origin, radius, surface);
    }

    fn parse_triangle(data: &Value) -> Triangle {
        let a = Scene::parse_vec3(&data["a"]);
        let b = Scene::parse_vec3(&data["b"]);
        let c = Scene::parse_vec3(&data["c"]);
        let surface = Scene::parse_surface(&data["surface"]);
        return Triangle::from_3_points_and_surface(&a, &b, &c, surface);
    }

    fn parse_quad(data: &Value) -> Quad {
        let a = Scene::parse_vec3(&data["a"]);
        let b = Scene::parse_vec3(&data["b"]);
        let c = Scene::parse_vec3(&data["c"]);
        let surface = Scene::parse_surface(&data["surface"]);
        return Quad::from_3_points_and_surface(&a, &b, &c, surface);
    }

    fn parse_vec3(data: &Value) -> Vector3<f32> {
        Vector3::new(
            data[0].as_f64().unwrap() as f32,
            data[1].as_f64().unwrap() as f32,
            data[2].as_f64().unwrap() as f32,
        )
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use epaint::Rgba;
    use nalgebra::Vector3;

    use crate::{
        intersect::Intersect,
        objects::{quad::Quad, sphere::Sphere, triangle},
        scene::Scene,
        surfaces::{diffuse::Diffuse, specular::Specular},
    };

    #[test]
    fn test_serde() {
        let mut objects: Vec<Arc<dyn Intersect>> = vec![Arc::new(Sphere::with_shader(
            Vector3::new(0.0, 0.0, 0.0),
            1.0,
            Arc::new(Diffuse::new(Rgba::from_rgb(0.5, 0.7, 0.0), 10)),
        ))];

        let tri_a = Vector3::new(0.0, 0.0, 1.0);
        let tri_b = Vector3::new(1.0, 0.0, 1.0);
        let tri_c = Vector3::new(1.0, 1.0, 1.0);

        objects.push(Arc::new(triangle::Triangle::from_3_points_and_surface(
            &tri_a,
            &tri_b,
            &tri_c,
            Arc::new(Specular::with_colour(Rgba::from_rgb(0.2, 0.2, 1.0))),
        )));
        let quad_a = Vector3::new(0.0, 0.0, 2.0);
        let quad_b = Vector3::new(1.0, 0.0, 2.0);
        let quad_c = Vector3::new(1.0, 1.0, 2.0);
        objects.push(Arc::new(Quad::from_3_points_and_surface(
            &quad_a,
            &quad_b,
            &quad_c,
            Arc::new(Specular::new()),
        )));

        let expected = Scene {
            objects,
            max_depth: 1,
        };

        assert_eq!(
            format!("{:?}", expected),
            format!("{:?}", Scene::from_json("jsons/ci.json"))
        );
    }
}
