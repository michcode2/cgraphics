use camera::Camera;
use eframe::egui::{self, Key, Rgba};
use renderer::Ray;
use scene::Scene;
mod camera;
mod intersect;
mod light;
mod renderer;
mod scene;
mod sphere;

fn main() -> eframe::Result {
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
    scene: Scene,
}

impl Default for RenderApp {
    fn default() -> Self {
        let width = 500;
        let height = 800;

        let row = (0..width)
            .map(|_| Rgba::from_gray(0.0))
            .collect::<Vec<Rgba>>();

        let buffer = (0..height).map(|_| row.clone()).collect::<Vec<Vec<Rgba>>>();

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
            scene: Scene::pondering_orbs(),
        }
    }
}

#[allow(deprecated)]
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
                        Key::ArrowLeft => self.camera.rotate_horizontal(0.01),
                        Key::ArrowRight => self.camera.rotate_horizontal(-0.01),
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

    fn update_buffer_sharedstate(&mut self) {
        //self.buffer = self.camera.create_buffer_parallel(self.scene.clone());
        self.buffer = self.camera.create_buffer(&self.scene);
    }
}
