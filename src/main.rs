mod raytracer;
mod structures;

use glam::{Vec3, Vec4};
use image::{ImageReader, Rgb};
use structures::{Background, Light, Material, Sphere};

fn main() {
    println!("Hello, world!");
    let background = ImageReader::open("envmap.jpg").unwrap().decode().unwrap();
    let mut spheres = Vec::new();
    let mut lights = Vec::new();
    let ivory = Material::new(
        Rgb([102, 102, 76]),
        Vec4::new(0.6, 0.3, 0.05, 0.0),
        50f32,
        1f32,
    );
    let red = Material::new(
        Rgb([76, 25, 25]),
        Vec4::new(0.9, 0.1, 0.0, 0.0),
        10f32,
        1f32,
    );
    let mirror = Material::new(
        Rgb([255, 255, 255]),
        Vec4::new(0.0, 10.0, 0.8, 0.0),
        1425f32,
        1f32,
    );
    let glass = Material::new(
        Rgb([150, 175, 200]),
        Vec4::new(0.0, 0.5, 0.1, 0.8),
        125f32,
        1.5f32,
    );
    spheres.push(Sphere::new(Vec3::new(-3f32, 0f32, -8f32), 2f32, ivory));
    spheres.push(Sphere::new(Vec3::new(-1f32, -1.5f32, -6f32), 2f32, ivory));
    spheres.push(Sphere::new(Vec3::new(1.5f32, -0.5f32, -9f32), 2f32, red));
    spheres.push(Sphere::new(Vec3::new(-1f32, 3.5f32, -7f32), 2f32, red));
    spheres.push(Sphere::new(Vec3::new(3f32, -3f32, -6f32), 2f32, mirror));
    spheres.push(Sphere::new(Vec3::new(-3f32, 3f32, -5f32), 2f32, glass));

    lights.push(Light::new(Vec3::new(5f32, 5f32, -2f32), 1.5));
    raytracer::render(&spheres, &lights, Background::new(background));
}
