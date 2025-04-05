mod raytracer;
mod structures;

use glam::{Vec3, Vec4};
use image::ImageReader;
use structures::{Background, Color, Light, Material, Scene, Sphere, Traceable};

fn main() {
    println!("Rendering image...");
    let background = ImageReader::open("envmap.jpg").unwrap().decode().unwrap();
    let mut objects: Vec<Box<dyn Traceable>> = Vec::new();
    let mut lights = Vec::new();
    let ivory = Material {
        diffuse_color: Color {
            r: 102,
            g: 102,
            b: 76,
        },
        albedo: Vec4::new(0.6, 0.3, 0.05, 0.0),
        specular_exponent: 50f32,
        refractive_index: 1f32,
    };
    let red = Material {
        diffuse_color: Color {
            r: 76,
            g: 25,
            b: 25,
        },
        albedo: Vec4::new(0.9, 0.1, 0.0, 0.0),
        specular_exponent: 10f32,
        refractive_index: 1f32,
    };
    let mirror = Material {
        diffuse_color: Color {
            r: 255,
            g: 255,
            b: 255,
        },
        albedo: Vec4::new(0.0, 10.0, 0.8, 0.0),
        specular_exponent: 1425f32,
        refractive_index: 1f32,
    };
    let glass = Material {
        diffuse_color: Color {
            r: 150,
            g: 175,
            b: 200,
        },
        albedo: Vec4::new(0.0, 0.5, 0.1, 0.8),
        specular_exponent: 125f32,
        refractive_index: 1.5f32,
    };
    objects.push(Box::new(Sphere {
        center: Vec3::new(-3f32, 0f32, -8f32),
        radius: 2f32,
        material: ivory,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(-1f32, -1.5f32, -6f32),
        radius: 2f32,
        material: ivory,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(1.5f32, -0.5f32, -9f32),
        radius: 2f32,
        material: red,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(-1f32, 3.5f32, -7f32),
        radius: 2f32,
        material: red,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(3f32, -3f32, -6f32),
        radius: 2f32,
        material: mirror,
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(-3f32, 3f32, -5f32),
        radius: 2f32,
        material: glass,
    }));

    lights.push(Light {
        position: Vec3::new(5f32, 5f32, -2f32),
        intensity: 1.5,
    });

    let main_scene = Scene { lights, objects };
    raytracer::render(main_scene, Background { image: background });
}
