use glam::Vec3;
use image::Rgb;

// TODO: Create a Scene struct which will hold meshes and lights.
// The scene_intersect function should be implemented for this struct as it will have direct access to all scene objects.
// Can support functions for adding/removing lights and objects.

pub struct Material {
    pub diffuse_color: Rgb<u8>,
}

impl Material {
    pub fn new(color: Rgb<u8>) -> Material {
        Material {
            diffuse_color: color,
        }
    }
}

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
