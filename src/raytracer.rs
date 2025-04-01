use std::f32::consts::PI;

use glam::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};

use crate::structures::{Material, Sphere};

impl Sphere {
    // For reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    pub fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<f32> {
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
        return Some(t0);
    }
}

pub fn scene_intersect<'a>(
    origin: &Vec3,
    direction: &Vec3,
    spheres: &'a Vec<Sphere>,
) -> Option<&'a Material> {
    let mut min_distance = f32::MAX;
    let mut material = &Material {
        diffuse_color: Rgb([0, 0, 0]),
    };
    for sphere in spheres {
        if let Some(dist) = sphere.ray_intersect(origin, direction) {
            if dist < min_distance {
                min_distance = dist;
                material = &sphere.material;
            }
        }
    }
    // TODO: Add variable for render distance?
    if min_distance < 1000f32 {
        return Some(material);
    }
    return None;
}

fn cast_ray(origin: &Vec3, direction: &Vec3, spheres: &Vec<Sphere>) -> Rgb<u8> {
    match scene_intersect(origin, direction, spheres) {
        None => Rgb([100, 100, 80]),
        Some(material) => material.diffuse_color,
    }
}

pub fn render(spheres: &Vec<Sphere>) {
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = 768;

    let mut frame_buffer: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let fov = PI / 2f32;

    for (x, y, pixel) in frame_buffer.enumerate_pixels_mut() {
        let x_pos = (2f32 * (x as f32 + 0.5) / IMAGE_WIDTH as f32 - 1f32)
            * f32::tan(fov / 2f32)
            * IMAGE_WIDTH as f32
            / IMAGE_HEIGHT as f32;
        let y_pos = -(2f32 * (y as f32 + 0.5) / IMAGE_HEIGHT as f32 - 1f32) * f32::tan(fov / 2f32);

        let direction = Vec3::normalize(Vec3::new(x_pos, y_pos, -1f32));
        *pixel = cast_ray(&Vec3::ZERO, &direction, spheres);
    }

    frame_buffer.save("out.png").unwrap();
}
