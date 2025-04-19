use eframe::egui::Rgba;
use nalgebra;

use crate::intersect::Intersect;
use crate::sphere::Sphere;

#[allow(non_snake_case)]
pub fn single_sphere(x: f32, y: f32) -> Rgba {
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-Sphere-intersection.html
    let C = nalgebra::Vector3::new(2.0, 0.0, 0.0); // center of Sphere
    let r = 0.5;
    let O = nalgebra::Vector3::new(0.0, 0.0, 0.0); // ray origin
    let D = nalgebra::Vector3::new(1.0, y, x);
    let L = C - O;
    let t_ca = L.dot(&D);

    if t_ca < 0.0 {
        return Rgba::from_gray(1.0);
    }

    let CLApp = O + D * t_ca; // closest approach
    let distance = (CLApp - C).norm();
    if distance < r {
        return Rgba::from_rgb(CLApp.x, CLApp.y, CLApp.z);
    }
    return Rgba::from_gray(1.0);
}

pub fn coordinates(x: f32, y: f32) -> Rgba {
    return Rgba::from_rgb(x, y, 0.0);
}

pub fn many_spheres(x: f32, y: f32) -> Rgba {
    let first_sphere = Sphere {
        origin: nalgebra::Vector3::new(15.0, 2.0, 3.0),
        radius: 1.0,
    };

    let second_sphere = Sphere {
        origin: nalgebra::Vector3::new(15.0, -2.0, 3.0),
        radius: 1.0,
    };

    let mut objects = vec![first_sphere, second_sphere];

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, -3.0, 0.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, 3.0, 0.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, 2.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, -2.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, -1.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, 1.0, -1.0),
        radius: 0.5,
    });

    objects.push(Sphere {
        origin: nalgebra::Vector3::new(15.0, 0.0, -1.0),
        radius: 0.5,
    });

    let direction = nalgebra::Vector3::new(2.0, y, x + 0.1);
    let direction = direction / direction.norm();
    let origin = nalgebra::Vector3::new(0.0, 0.0, 0.0);

    let mut total = vec![0_u8; 3];

    objects
        .into_iter()
        .map(|obj| obj.test_intersection(&origin, &direction))
        .map(|c| c.to_srgba_unmultiplied())
        .for_each(|[r, g, b, _]| {
            total[0] = total[0].saturating_add(r);
            total[1] = total[1].saturating_add(g);
            total[2] = total[2].saturating_add(b);
        });
    Rgba::from_srgba_unmultiplied(total[0], total[1], total[2], 255)
}
