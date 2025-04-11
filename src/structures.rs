use glam::{Vec3, Vec4};
use image::{DynamicImage, Rgb};
use serde::{Deserialize, Serialize};

pub struct Scene {
    pub lights: Vec<Light>,
    pub objects: Vec<Box<dyn Traceable>>,
    pub background: Option<DynamicImage>,
}

pub trait Traceable {
    fn intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersection>;
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Traceable for AABB {
    fn intersection(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersection> {
        let inv_dir = Vec3::new(
            if direction.x == 0.0 {
                f32::INFINITY
            } else {
                1.0 / direction.x
            },
            if direction.y == 0.0 {
                f32::INFINITY
            } else {
                1.0 / direction.y
            },
            if direction.z == 0.0 {
                f32::INFINITY
            } else {
                1.0 / direction.z
            },
        );

        let t1 = (self.min.x - origin.x) * inv_dir.x;
        let t2 = (self.max.x - origin.x) * inv_dir.x;
        let t3 = (self.min.y - origin.y) * inv_dir.y;
        let t4 = (self.max.y - origin.y) * inv_dir.y;
        let t5 = (self.min.z - origin.z) * inv_dir.z;
        let t6 = (self.max.z - origin.z) * inv_dir.z;

        let tmin_x = t1.min(t2);
        let tmax_x = t1.max(t2);
        let tmin_y = t3.min(t4);
        let tmax_y = t3.max(t4);
        let tmin_z = t5.min(t6);
        let tmax_z = t5.max(t6);

        let overall_tmin = tmin_x.max(tmin_y).max(tmin_z);
        let overall_tmax = tmax_x.min(tmax_y).min(tmax_z);

        if overall_tmin > overall_tmax || overall_tmax < 0.0 {
            return None;
        }

        let t_hit = if overall_tmin < 0.0 {
            overall_tmax
        } else {
            overall_tmin
        };

        let hit_point = *origin + *direction * t_hit;

        let center = (self.min + self.max) * 0.5;
        let point_relative_to_center = hit_point - center;
        let half_extents = (self.max - self.min) * 0.5;

        let epsilon = 1e-4;

        let mut normal = Vec3::ZERO;

        if (point_relative_to_center.x / half_extents.x).abs() > 1.0 - epsilon {
            normal = Vec3::new(point_relative_to_center.x.signum(), 0.0, 0.0);
        } else if (point_relative_to_center.y / half_extents.y).abs() > 1.0 - epsilon {
            normal = Vec3::new(0.0, point_relative_to_center.y.signum(), 0.0);
        } else if (point_relative_to_center.z / half_extents.z).abs() > 1.0 - epsilon {
            normal = Vec3::new(0.0, 0.0, point_relative_to_center.z.signum());
        }

        Some(Intersection {
            point: hit_point,
            normal,
            material: self.material,
        })
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
