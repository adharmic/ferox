mod raytracer;
mod structures;

use glam::Vec3;
use image::Rgb;
use structures::{Material, Sphere};

fn main() {
    println!("Hello, world!");
    let mut spheres = Vec::new();
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
    raytracer::render(&spheres);
}
