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
        Vec3::new(-3f32, 0f32, -16f32),
        2f32,
        Material::new(Rgb([255, 0, 0])),
    ));
    spheres.push(Sphere::new(
        Vec3::new(-1f32, -1.5f32, -12f32),
        2f32,
        Material::new(Rgb([0, 255, 0])),
    ));
    spheres.push(Sphere::new(
        Vec3::new(1.5f32, -0.5f32, -18f32),
        2f32,
        Material::new(Rgb([0, 0, 255])),
    ));

    lights.push(Light::new(Vec3::new(-20f32, -20f32, -20f32), 0.5));
    lights.push(Light::new(Vec3::new(20f32, 10f32, -10f32), 1.5));
    raytracer::render(&spheres, &lights);
}
