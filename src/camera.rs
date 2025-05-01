use eframe::egui::Rgba;
use nalgebra::Vector3;

use crate::{renderer::Ray, scene::Scene};

pub struct Camera {
    pub location: Ray,
    pub width: u32,
    pub height: u32,
}

impl Camera {
    // calls the render function on the provided scene for eah pixel and put it where it should be
    pub fn create_buffer(&self, scene: &Scene) -> Vec<Vec<Rgba>> {
        // init the buffer to pure black
        let mut buffer: Vec<Vec<Rgba>> =
            vec![vec![Rgba::from_gray(0.0); self.height as usize]; self.width as usize];

        for x in 0..self.width {
            for y in 0..self.height {
                // each dimension on screen should be a point from -1 to 1
                let x_normalised = ((-2.0 * x as f32) / self.width as f32) + 1.0;
                let y_normalised = ((2.0 * y as f32) / self.height as f32) - 1.0;

                let pixel_direction = nalgebra::Vector3::new(
                    -y_normalised * self.get_direction_horizontal().sin(), // who up rotating
                    y_normalised * self.get_direction_horizontal().cos(),  // their matrix
                    x_normalised, // this will need to get an update when the camera can change pitch and it is not defined as the z coordinate
                );

                let pixel_ray = Ray::new(
                    self.location.origin,
                    pixel_direction + self.location.direction,
                );

                // do the calculations and put it in the buffer
                let color = scene.test_intersections(pixel_ray, 0).colour;
                buffer[x as usize][y as usize] = color;
            }
        }
        buffer
    }

    pub fn rotate_horizontal(&mut self, dtheta: f32) {
        // get how much of the vector is in the horizontal plan e
        let r = (self.location.direction.x.powi(2) + self.location.direction.y.powi(2)).sqrt();

        let theta_1 = self.get_direction_horizontal() - dtheta;

        let x_1 = theta_1.cos() * r;
        let y_1 = theta_1.sin() * r;

        self.location.direction.x = x_1;
        self.location.direction.y = y_1;
    }

    fn get_direction_horizontal(&self) -> f32 {
        let x = self.location.direction.x;
        let y = self.location.direction.y;

        // trust me this is right
        y.atan2(x)
    }

    pub fn move_by(&mut self, direction: Vector3<f32>) {
        // rawdogging the rotation matricies (sorry future boss)
        self.location.origin.x += direction.x * self.get_direction_horizontal().cos()
            - direction.y * self.get_direction_horizontal().sin();
        self.location.origin.y += direction.x * self.get_direction_horizontal().sin()
            + direction.y * self.get_direction_horizontal().cos();
        self.location.origin.z += direction.z;
    }

    /*
    maybe useful if i lock in and make parallel work properly

    pub fn create_buffer_parallel(&self, scene: Scene) -> Vec<Vec<Rgba>> {
        let mut buffer: Vec<Vec<Rgba>> =
            vec![vec![Rgba::from_gray(0.0); self.height as usize]; self.width as usize];
        let scene_pointer = Arc::new(RwLock::new(scene));
        let mut jobs = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                let dim_small = self.width.min(self.height) as f32;
                let x_normalised = ((-2.0 * x as f32) / dim_small) + 1.0;
                let y_normalised = ((2.0 * y as f32) / dim_small) - 1.0;

                let pixel_direction = nalgebra::Vector3::new(0.0, y_normalised, x_normalised);

                let pixel_ray = Ray::new(
                    self.location.origin,
                    pixel_direction + self.location.direction,
                );

                let thread_scene_pointer = Arc::clone(&scene_pointer);

                let job = move || {
                    return thread_scene_pointer
                        .read()
                        .unwrap()
                        .test_intersections(pixel_ray, 0);
                };
                jobs.push((x as usize, y as usize, job));
            }
        }

        let mut in_progress = vec![];

        while jobs.len() > 0 {
            while in_progress.len() < 8 {
                let job = jobs.pop().unwrap();
                in_progress.push(thread::spawn(move || return (job.0, job.1, job.2())));
            }
            for i in 0..in_progress.len() - 1 {
                if in_progress[i].is_finished() {
                    let (x, y, intersection) = in_progress.remove(i).join().unwrap();
                    buffer[x][y] = intersection.colour;
                    break;
                }
            }
        }
        buffer
    }*/
}
