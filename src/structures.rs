use glam::{Vec3, Vec4};
use image::{DynamicImage, Rgb};

// TODO: Create a Scene struct which will hold meshes and lights.
// The scene_intersect function should be implemented for this struct as it will have direct access to all scene objects.
// Can support functions for adding/removing lights and objects.

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

impl Light {
    pub fn new(position: Vec3, intensity: f32) -> Light {
        Light {
            position,
            intensity,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Rgb<u8>,
    pub albedo: Vec4,
    pub specular_exponent: f32,
    pub refractive_index: f32,
}

impl Material {
    pub fn new(
        color: Rgb<u8>,
        albedo: Vec4,
        specular_exponent: f32,
        refractive_index: f32,
    ) -> Material {
        Material {
            diffuse_color: color,
            albedo,
            specular_exponent,
            refractive_index,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct Background {
    pub image: DynamicImage,
}

impl Background {
    pub fn new(image: DynamicImage) -> Background {
        Background { image }
    }
}
