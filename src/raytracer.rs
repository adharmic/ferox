use std::f32::consts::PI;

use glam::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};

use crate::structures::{Intersection, Light, Material, Sphere};

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

pub fn scene_intersect(
    origin: &Vec3,
    direction: &Vec3,
    spheres: &[Sphere],
) -> Option<Intersection> {
    let mut closest_intersection: Option<Intersection> = None;
    let mut min_distance = f32::MAX;
    for sphere in spheres {
        if let Some(sphere_distance) = sphere.ray_intersect(origin, direction) {
            // TODO: Add variable for render distance?
            if sphere_distance < min_distance && sphere_distance < 1000f32 {
                min_distance = sphere_distance;

                let hit = origin + direction * sphere_distance;
                let normal = (hit - sphere.center).normalize();
                let material = sphere.material;
                closest_intersection = Some(Intersection {
                    point: hit,
                    normal,
                    material,
                })
            }
        }
    }
    return closest_intersection;
}

fn cast_ray(origin: &Vec3, direction: &Vec3, spheres: &[Sphere], lights: &[Light]) -> Rgb<u8> {
    let hit: Vec3;
    let normal: Vec3;
    let material: Material;

    match scene_intersect(origin, direction, spheres) {
        Some(intersection) => {
            hit = intersection.point;
            normal = intersection.normal;
            material = intersection.material;
        }
        None => return Rgb([100, 100, 80]),
    }
    return calculate_lighting_color(&material, lights, &hit, &normal);
}

fn calculate_lighting_color(
    material: &Material,
    lights: &[Light],
    hit: &Vec3,
    normal: &Vec3,
) -> Rgb<u8> {
    let mut diffuse_light_intensity = 0f32;
    for light in lights {
        let light_direction: Vec3 = (light.position - hit).normalize();
        diffuse_light_intensity += light.intensity * f32::max(0f32, light_direction.dot(*normal));
    }
    return Rgb([
        (material.diffuse_color.0[0] as f32 * diffuse_light_intensity) as u8,
        (material.diffuse_color.0[1] as f32 * diffuse_light_intensity) as u8,
        (material.diffuse_color.0[2] as f32 * diffuse_light_intensity) as u8,
    ]);
}

pub fn render(spheres: &Vec<Sphere>, lights: &[Light]) {
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
        *pixel = cast_ray(&Vec3::ZERO, &direction, spheres, lights);
    }

    frame_buffer.save("out.png").unwrap();
}
