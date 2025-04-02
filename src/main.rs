mod raytracer;
mod structures;

use glam::Vec3;
use image::Rgb;
use structures::{Light, Material, Sphere};

fn main() {
    println!("Hello, world!");
    let mut spheres = Vec::new();
    let mut lights = Vec::new();
    spheres.push(Sphere::new(
        Vec3::new(-3f32, 0f32, -8f32),
        2f32,
        Material::new(Rgb([255, 0, 0])),
    ));
    spheres.push(Sphere::new(
        Vec3::new(-1f32, -1.5f32, -6f32),
        2f32,
        Material::new(Rgb([0, 255, 0])),
    ));
    spheres.push(Sphere::new(
        Vec3::new(1.5f32, -0.5f32, -9f32),
        2f32,
        Material::new(Rgb([0, 0, 255])),
    ));

    lights.push(Light::new(Vec3::new(5f32, 5f32, -2f32), 1.5));
    raytracer::render(&spheres, &lights);
}
