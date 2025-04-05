use glam::{Vec3, Vec4};
use image::{DynamicImage, Rgb};

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Traceable>>,
}

pub trait Traceable {
    fn intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersection>;
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse_color: Color,
    pub albedo: Vec4,
    pub specular_exponent: f32,
    pub refractive_index: f32,
}

pub struct Intersection {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub struct Background {
    pub image: DynamicImage,
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Traceable for Sphere {
    fn intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersection> {
        let l = self.center - origin;
        let tca = l.dot(*direction);
        if tca < 0f32 {
            return None;
        }
        let d = f32::sqrt(l.dot(l) - tca * tca);
        if d < 0f32 || d > self.radius {
            return None;
        }
        let thc = f32::sqrt(self.radius * self.radius - d * d);
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0f32 {
            t0 = t1;
        }
        if t0 < 0f32 {
            return None;
        }
        let intersection_point = origin + direction * t0;
        let intersection_normal = (intersection_point - self.center).normalize();
        return Some(Intersection {
            point: intersection_point,
            normal: intersection_normal,
            material: self.material,
        });
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_vector(vector: Vec3) -> Color {
        Color {
            r: (f32::clamp(vector.x, 0f32, 1f32) * 255f32) as u8,
            g: (f32::clamp(vector.y, 0f32, 1f32) * 255f32) as u8,
            b: (f32::clamp(vector.z, 0f32, 1f32) * 255f32) as u8,
        }
    }
    pub fn as_vector(&self) -> Vec3 {
        return Vec3::new(
            self.r as f32 / 255f32,
            self.g as f32 / 255f32,
            self.b as f32 / 255f32,
        );
    }

    pub fn as_rgb(&self) -> Rgb<u8> {
        return Rgb([self.r, self.g, self.b]);
    }
}

// TODO: Finish implementing plane, box, and triangle-ray intersections.
// pub struct Plane {
//     point: Vec3,
//     normal: Vec3,
//     material: Material,
// }

// impl Plane {
//     pub fn new(point: Vec3, normal: Vec3, material: Material) -> Plane {
//         Plane {
//             point,
//             normal,
//             material,
//         }
//     }

//     pub fn intersects(&self, origin: &Vec3, direction: &Vec3) -> bool {
//         let denominator = self.normal.dot(*direction);
//         if denominator > 0.0001f32 {
//             let hit = self.point - origin;
//             let distance = hit.dot(self.normal) / denominator;
//             return distance > 0f32;
//         }
//         return false;
//     }
// }
