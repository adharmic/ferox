use std::{f32::consts::PI, mem::swap};

use glam::Vec3;
use image::{GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};

use crate::structures::{Background, Intersection, Light, Material, Sphere};

impl Sphere {
    // For reference: https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html
    pub fn calculate_intersection_distance(&self, origin: &Vec3, direction: &Vec3) -> Option<f32> {
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

    pub fn calculate_intersection_position(
        self,
        origin: &Vec3,
        direction: &Vec3,
        distance: f32,
    ) -> Intersection {
        let point = origin + direction * distance;
        let normal = (point - self.center).normalize();
        return Intersection {
            point,
            normal,
            material: self.material,
        };
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
        if let Some(sphere_distance) = sphere.calculate_intersection_distance(origin, direction) {
            // TODO: Add variable for render distance?
            if sphere_distance < min_distance && sphere_distance < 1000f32 {
                min_distance = sphere_distance;
                closest_intersection = Some(sphere.calculate_intersection_position(
                    origin,
                    direction,
                    sphere_distance,
                ));
            }
        }
    }
    return closest_intersection;
}

fn cast_ray(
    origin: &Vec3,
    direction: &Vec3,
    spheres: &[Sphere],
    lights: &[Light],
    recursive_depth: u8,
    background: &Background,
) -> Rgb<u8> {
    let hit: Vec3;
    let normal: Vec3;
    let material: Material;
    let background_color = calculate_background_color(direction, background);

    if recursive_depth > 4 {
        return background_color;
    }

    match scene_intersect(origin, direction, spheres) {
        Some(intersection) => {
            hit = intersection.point;
            normal = intersection.normal;
            material = intersection.material;
        }
        None => return background_color,
    }

    return calculate_color(
        &material,
        lights,
        &hit,
        &normal,
        &direction,
        spheres,
        recursive_depth,
        background,
    );
}

fn calculate_background_color(direction: &Vec3, background: &Background) -> Rgb<u8> {
    if background.image.dimensions() == (0, 0) {
        return Rgb([50, 180, 200]);
    }

    let norm_direction = direction.normalize();

    let px = norm_direction.x;
    let py = norm_direction.y;
    let pz = norm_direction.z;

    let phi = f32::acos(f32::clamp(py, -1.0, 1.0));
    let theta = f32::atan2(pz, px);

    let u = (theta + PI) / (2.0 * PI);
    let v = phi / PI;

    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let tex_width = background.image.dimensions().0;
    let tex_height = background.image.dimensions().1;

    let tex_x = u * (tex_width - 1) as f32;
    let tex_y = v * (tex_height - 1) as f32; // Try: (1.0 - v) * (tex_height - 1) as f32; if flipped

    return background
        .image
        .get_pixel(tex_x as u32, tex_y as u32)
        .to_rgb();
}

fn calculate_color(
    material: &Material,
    lights: &[Light],
    hit: &Vec3,
    normal: &Vec3,
    direction: &Vec3,
    spheres: &[Sphere],
    recursive_depth: u8,
    background: &Background,
) -> Rgb<u8> {
    let reflection_color = calculate_reflection_color(
        direction,
        normal,
        hit,
        spheres,
        lights,
        recursive_depth,
        background,
    );
    let refraction_color = calculate_refraction_color(
        direction,
        normal,
        material,
        hit,
        spheres,
        lights,
        recursive_depth,
        background,
    );

    let reflection_vector = calculate_vector_from_color(reflection_color);
    let refraction_vector = calculate_vector_from_color(refraction_color);

    let mut diffuse_light_intensity = 0f32;
    let mut specular_light_intensity = 0f32;
    let mut calculated_color = calculate_vector_from_color(material.diffuse_color);
    for light in lights {
        let light_direction = (light.position - hit).normalize();
        let light_distance = (light.position - hit).length();
        let shadow_origin = calculate_ray_offset(&light_direction, normal, hit);
        if let Some(shadow_intersection) =
            scene_intersect(&shadow_origin, &light_direction, spheres)
        {
            if (shadow_intersection.point - shadow_origin).length() < light_distance {
                continue;
            }
        }
        diffuse_light_intensity += light.intensity * f32::max(0f32, light_direction.dot(*normal));
        specular_light_intensity += f32::powf(
            f32::max(
                0f32,
                -calculate_reflection_angle(&(-light_direction), &normal).dot(*direction),
            ),
            material.specular_exponent * light.intensity,
        )
    }

    calculated_color = calculated_color * diffuse_light_intensity * material.albedo[0]
        + Vec3::ONE * specular_light_intensity * material.albedo[1]
        + reflection_vector * material.albedo[2]
        + refraction_vector * material.albedo[3];

    return Rgb([
        u8::clamp((calculated_color[0] * 255f32) as u8, 0, 255),
        u8::clamp((calculated_color[1] * 255f32) as u8, 0, 255),
        u8::clamp((calculated_color[2] * 255f32) as u8, 0, 255),
    ]);
}

fn calculate_vector_from_color(color: Rgb<u8>) -> Vec3 {
    return Vec3::new(
        color.0[0] as f32 / 255 as f32,
        color.0[1] as f32 / 255 as f32,
        color.0[2] as f32 / 255 as f32,
    );
}

fn calculate_refraction_color(
    incident: &Vec3,
    normal: &Vec3,
    material: &Material,
    hit: &Vec3,
    spheres: &[Sphere],
    lights: &[Light],
    recursive_depth: u8,
    background: &Background,
) -> Rgb<u8> {
    let refraction_direction =
        calculate_refraction_angle(incident, normal, material.refractive_index);
    let refraction_origin = calculate_ray_offset(&refraction_direction, normal, hit);
    return cast_ray(
        &refraction_origin,
        &refraction_direction,
        spheres,
        lights,
        recursive_depth + 1,
        background,
    );
}

fn calculate_ray_offset(direction: &Vec3, normal: &Vec3, hit: &Vec3) -> Vec3 {
    let offset = 0.0001f32;
    return match direction.dot(*normal) < 0f32 {
        true => hit - offset,
        false => hit + offset,
    };
}

fn calculate_reflection_color(
    direction: &Vec3,
    normal: &Vec3,
    hit: &Vec3,
    spheres: &[Sphere],
    lights: &[Light],
    recursive_depth: u8,
    background: &Background,
) -> Rgb<u8> {
    let reflection_direction = calculate_reflection_angle(direction, normal);
    let reflection_origin: Vec3;
    if reflection_direction.dot(*normal) < 0f32 {
        reflection_origin = hit - 0.0001f32;
    } else {
        reflection_origin = hit + 0.0001f32;
    }
    return cast_ray(
        &reflection_origin,
        &reflection_direction,
        spheres,
        lights,
        recursive_depth + 1,
        background,
    );
}

pub fn render(spheres: &Vec<Sphere>, lights: &[Light], background: Background) {
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
        *pixel = cast_ray(&Vec3::ZERO, &direction, spheres, lights, 0, &background);
    }

    frame_buffer.save("out.png").unwrap();
}

fn calculate_reflection_angle(incident: &Vec3, normal: &Vec3) -> Vec3 {
    return incident - normal * 2f32 * incident.dot(*normal);
}

fn calculate_refraction_angle(incident: &Vec3, normal: &Vec3, refractive_index: f32) -> Vec3 {
    let mut cosi = -f32::max(-1f32, f32::min(1f32, incident.dot(*normal)));
    let mut etai = 1f32;
    let mut etat = refractive_index;
    let mut n = normal;
    let normal_inverse = -normal;
    let eta: f32;
    if cosi < 0f32 {
        cosi = -cosi;
        swap(&mut etai, &mut etat);
        n = &normal_inverse;
    }
    eta = etai / etat;
    let k = 1f32 - eta * eta * (1f32 - cosi * cosi);
    if k < 0f32 {
        return Vec3::ZERO;
    }
    return incident * eta + n * (eta * cosi - f32::sqrt(k));
}
