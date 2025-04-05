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
