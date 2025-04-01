use std::f32::consts::PI;

use glam::Vec3;
use image::{ImageBuffer, Rgb, RgbImage};

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    // For reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> bool {
        let l = self.center - origin;
        let tca = l.dot(*direction);
        if tca < 0f32 {
            return false;
        }
        let d = f32::sqrt(l.dot(l) - tca * tca);
        if d < 0f32 || d > self.radius {
            return false;
        }
        let thc = f32::sqrt(self.radius * self.radius - d * d);
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0f32 {
            t0 = t1;
        }
        if t0 < 0f32 {
            return false;
        }
        return true;
    }
}

fn render(sphere: &Sphere) {
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
        *pixel = cast_ray(&Vec3::ZERO, &direction, sphere);
    }

    frame_buffer.save("out.png").unwrap();
}

fn cast_ray(origin: &Vec3, direction: &Vec3, sphere: &Sphere) -> Rgb<u8> {
    if !sphere.ray_intersect(origin, direction) {
        return Rgb([
            (0.2 * 255f32) as u8,
            (0.7 * 255f32) as u8,
            (0.8 * 255f32) as u8,
        ]);
    }
    return Rgb([
        (0.4 * 255f32) as u8,
        (0.4 * 255f32) as u8,
        (0.3 * 255f32) as u8,
    ]);
}

fn main() {
    println!("Hello, world!");
    render(&Sphere::new(Vec3::new(-3f32, 0f32, -16f32), 2f32));
}
