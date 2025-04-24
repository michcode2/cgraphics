use camera::Camera;
use eframe::egui::{self, Key, Rgba};
use renderer::Ray;
use scene::Scene;
mod camera;
mod intersect;
mod renderer;
mod scene;
mod sphere;

fn main() -> eframe::Result {
    println!("Hello, world!");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 820.0]),
        ..Default::default()
    };
    eframe::run_native(
        "renderer",
        options,
        Box::new(|_| {
            // This gives us image support:
            Ok(Box::<RenderApp>::default())
        }),
    )
}

struct RenderApp {
    buffer: Vec<Vec<Rgba>>,
    camera: Camera,
    scene: Scene<sphere::Sphere>,
}

impl Default for RenderApp {
    fn default() -> Self {
        let width = 800;
        let height = 800;

        let row = (0..height)
            .map(|_| Rgba::from_gray(0.0))
            .collect::<Vec<Rgba>>();

        let buffer = (0..width).map(|_| row.clone()).collect::<Vec<Vec<Rgba>>>();

        let ray_location = nalgebra::Vector3::new(-10.0, 0.0, 0.0);
        let ray_direction = nalgebra::Vector3::new(1.0, 0.0, 0.0);
        let origin_ray = Ray::new_preserve(ray_location, ray_direction);

        let camera = Camera {
            location: origin_ray,
            width,
            height,
        };

        RenderApp {
            buffer,
            camera,
            scene: Scene::<sphere::Sphere>::new_test(),
        }
    }
}

impl eframe::App for RenderApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_buffer_sharedstate();
            let img =
                egui_extras::image::RetainedImage::from_color_image("text", self.buffer_to_image());
            img.show(ui);

            ctx.input(|inputs| {
                for pressed in &inputs.keys_down {
                    match pressed {
                        Key::W => self.camera.location.origin.x += 0.1,
                        Key::S => self.camera.location.origin.x -= 0.1,
                        Key::A => self.camera.location.origin.y -= 0.1,
                        Key::D => self.camera.location.origin.y += 0.1,
                        Key::Z => self.camera.location.origin.z += 0.1,
                        Key::X => self.camera.location.origin.z -= 0.1,
                        Key::ArrowLeft => {
                            let x = self.camera.location.direction.x;
                            let y = self.camera.location.direction.y;

                            let theta = y.atan2(x);
                            let r = (x.powi(2) + y.powi(2)).sqrt();

                            let theta_1 = theta - 0.01;

                            let x_1 = theta_1.cos() * r;
                            let y_1 = theta_1.sin() * r;

                            self.camera.location.direction.x = x_1;
                            self.camera.location.direction.y = y_1;
                            println!("{:?}", self.camera.location.direction);
                        }
                        Key::ArrowRight => {
                            let x = self.camera.location.direction.x;
                            let y = self.camera.location.direction.y;

                            let theta = y.atan2(x);
                            let r = (x.powi(2) + y.powi(2)).sqrt();

                            let theta_1 = theta + 0.01;

                            let x_1 = theta_1.cos() * r;
                            let y_1 = theta_1.sin() * r;

                            self.camera.location.direction.x = x_1;
                            self.camera.location.direction.y = y_1;
                            println!("{:?}", self.camera.location.direction);
                        }
                        _ => (),
                    }
                }
            });
        });
    }
}

impl RenderApp {
    fn buffer_to_image(&self) -> egui::ColorImage {
        let mut flattened = vec![];
        for row in &self.buffer {
            for pixel in row {
                let values = pixel.to_srgba_unmultiplied();
                for v in values {
                    flattened.push(v);
                }
            }
        }
        egui::ColorImage::from_rgba_unmultiplied(
            [self.buffer[0].len(), self.buffer.len()],
            &flattened.as_slice(),
        )
    }

    fn update_buffer(&mut self) {
        let width = self.buffer[0].len();
        let height = self.buffer.len();
        for x in 0..width {
            for y in 0..height {
                let x_normalised = ((-2.0 * x as f32) / width as f32) + 1.0;
                let y_normalised = ((2.0 * y as f32) / height as f32) - 1.0;
                self.buffer[x][y] = renderer::many_spheres(x_normalised, y_normalised);
                //self.buffer[x][y] = renderer::coordinates(x_normalised, y_normalised);
            }
        }
    }

    fn update_buffer_sharedstate(&mut self) {
        self.buffer = self.camera.create_buffer_parallel(self.scene.clone());
        //self.buffer = self.camera.create_buffer(&self.scene);
    }
}
