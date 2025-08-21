use epaint::Rgba;

use crate::{common_maths::maths, renderer::Ray, surfaces::Surface};

#[derive(Clone, Copy, Debug)]
pub struct Specular {
    colour: Rgba,
}

impl Surface for Specular {
    fn get_value(&self, other: Rgba) -> epaint::Rgba {
        return self.colour + other;
    }

    fn request_rays(&self, normal_ray: &Ray, incoming_ray: &Ray) -> Vec<Ray> {
        let v = vec![maths::reflected_ray(normal_ray, incoming_ray)];
        //println!("{:?}", v);
        v
    }

    fn intersections_to_colour(&self, rays: Vec<crate::intersect::TestIntersectionResult>) -> Rgba {
        //println!("{:?}", rays);
        rays[0].0.colour.multiply(0.5) + self.colour.multiply(0.5)
        //self.colour
    }
}

impl Specular {
    pub fn new() -> Specular {
        Specular {
            colour: Rgba::from_gray(0.1),
        }
    }
    pub fn with_colour(colour: Rgba) -> Specular {
        Specular { colour }
    }
}
