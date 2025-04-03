mod raytracer;
mod structures;

use glam::Vec3;
use image::Rgb;
use structures::{Light, Material, Sphere};

fn main() {
    println!("Hello, world!");
    let mut spheres = Vec::new();
    let mut lights = Vec::new();
    let ivory = Material::new(
        Rgb([102, 102, 76]),
        Vec3 {
            x: 0.6,
            y: 0.3,
            z: 0.1,
        },
        50f32,
    );
    let red = Material::new(
        Rgb([76, 25, 25]),
        Vec3 {
            x: 0.9,
            y: 0.1,
            z: 0.0,
        },
        10f32,
    );
    let mirror = Material::new(
        Rgb([255, 255, 255]),
        Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.8,
        },
        1425f32,
    );
    spheres.push(Sphere::new(Vec3::new(-3f32, 0f32, -8f32), 2f32, ivory));
    spheres.push(Sphere::new(Vec3::new(-1f32, -1.5f32, -6f32), 2f32, ivory));
    spheres.push(Sphere::new(Vec3::new(1.5f32, -0.5f32, -9f32), 2f32, red));
    spheres.push(Sphere::new(Vec3::new(-1f32, 3.5f32, -7f32), 2f32, red));
    spheres.push(Sphere::new(Vec3::new(3f32, -3f32, -6f32), 2f32, mirror));

    lights.push(Light::new(Vec3::new(5f32, 5f32, -2f32), 1.5));
    raytracer::render(&spheres, &lights);
}
