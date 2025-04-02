mod raytracer;
mod structures;

use glam::{Vec2, Vec3};
use image::Rgb;
use structures::{Light, Material, Sphere};

fn main() {
    println!("Hello, world!");
    let mut spheres = Vec::new();
    let mut lights = Vec::new();
    let ivory = Material::new(Rgb([102, 102, 76]), Vec2 { x: 0.6, y: 0.3 }, 50f32);
    let red_rubber = Material::new(Rgb([76, 25, 25]), Vec2 { x: 0.9, y: 0.1 }, 10f32);
    spheres.push(Sphere::new(Vec3::new(-3f32, 0f32, -8f32), 2f32, ivory));
    spheres.push(Sphere::new(Vec3::new(-1f32, -1.5f32, -6f32), 2f32, ivory));
    spheres.push(Sphere::new(
        Vec3::new(1.5f32, -0.5f32, -9f32),
        2f32,
        red_rubber,
    ));

    lights.push(Light::new(Vec3::new(5f32, 5f32, -2f32), 1.5));
    raytracer::render(&spheres, &lights);
}
