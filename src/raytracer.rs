use std::{f32::consts::PI, mem::swap};

use glam::Vec3;
use image::{GenericImageView, ImageBuffer, Pixel, RgbImage};

use crate::structures::{Color, Intersection, Material, Scene};

pub fn scene_intersect(scene: &Scene, origin: &Vec3, direction: &Vec3) -> Option<Intersection> {
    let mut closest_intersection: Option<Intersection> = None;
    let mut min_distance = f32::MAX;
    for object in &scene.objects {
        if let Some(intersection) = object.intersection(origin, direction) {
            // TODO: Add variable for render distance?
            let intersection_distance = (intersection.point - origin).length();
            if intersection_distance < min_distance && intersection_distance < 1000f32 {
                min_distance = intersection_distance;
                closest_intersection = Some(intersection);
            }
        }
    }
    return closest_intersection;
}

fn cast_ray(scene: &Scene, origin: &Vec3, direction: &Vec3, recursive_depth: u8) -> Color {
    let hit: Vec3;
    let normal: Vec3;
    let material: Material;
    let background_color = background_color(direction, scene);

    if recursive_depth > 4 {
        return background_color;
    }

    match scene_intersect(scene, origin, direction) {
        Some(intersection) => {
            hit = intersection.point;
            normal = intersection.normal;
            material = intersection.material;
        }
        None => return background_color,
    }

    return color(
        &scene,
        &material,
        &hit,
        &normal,
        &direction,
        recursive_depth,
    );
}

fn background_color(direction: &Vec3, scene: &Scene) -> Color {
    if let Some(background) = &scene.background {
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

        let tex_width = background.dimensions().0;
        let tex_height = background.dimensions().1;

        let tex_x = u * (tex_width - 1) as f32;
        let tex_y = v * (tex_height - 1) as f32; // Try: (1.0 - v) * (tex_height - 1) as f32; if flipped

        let pixel_color = background.get_pixel(tex_x as u32, tex_y as u32).to_rgb();

        return Color {
            r: pixel_color.0[0],
            g: pixel_color.0[1],
            b: pixel_color.0[2],
        };
    }

    return Color {
        r: 50,
        g: 180,
        b: 200,
    };
}

fn color(
    scene: &Scene,
    material: &Material,
    hit: &Vec3,
    normal: &Vec3,
    direction: &Vec3,
    recursive_depth: u8,
) -> Color {
    let reflection_color = reflection_color(scene, direction, normal, hit, recursive_depth);
    let refraction_color =
        refraction_color(scene, direction, normal, material, hit, recursive_depth);

    let reflection_vector = reflection_color.as_vector();
    let refraction_vector = refraction_color.as_vector();

    let mut diffuse_light_intensity = 0f32;
    let mut specular_light_intensity = 0f32;
    let mut calculated_color = material.diffuse_color.as_vector();
    for light in &scene.lights {
        let light_direction = (light.position - hit).normalize();
        let light_distance = (light.position - hit).length();
        let shadow_origin = ray_offset(&light_direction, normal, hit);
        if let Some(shadow_intersection) = scene_intersect(scene, &shadow_origin, &light_direction)
        {
            if (shadow_intersection.point - shadow_origin).length() < light_distance {
                continue;
            }
        }
        diffuse_light_intensity += light.intensity * f32::max(0f32, light_direction.dot(*normal));
        specular_light_intensity += f32::powf(
            f32::max(
                0f32,
                -reflection_angle(&(-light_direction), &normal).dot(*direction),
            ),
            material.specular_exponent * light.intensity,
        )
    }

    calculated_color = calculated_color * diffuse_light_intensity * material.albedo[0]
        + Vec3::ONE * specular_light_intensity * material.albedo[1]
        + reflection_vector * material.albedo[2]
        + refraction_vector * material.albedo[3];

    return Color::from_vector(calculated_color);
}

fn refraction_color(
    scene: &Scene,
    incident: &Vec3,
    normal: &Vec3,
    material: &Material,
    hit: &Vec3,
    recursive_depth: u8,
) -> Color {
    let refraction_direction = refraction_angle(incident, normal, material.refractive_index);
    let refraction_origin = ray_offset(&refraction_direction, normal, hit);
    return cast_ray(
        scene,
        &refraction_origin,
        &refraction_direction,
        recursive_depth + 1,
    );
}

fn ray_offset(direction: &Vec3, normal: &Vec3, hit: &Vec3) -> Vec3 {
    let offset = 0.0001f32;
    return match direction.dot(*normal) < 0f32 {
        true => hit - offset,
        false => hit + offset,
    };
}

fn reflection_color(
    scene: &Scene,
    direction: &Vec3,
    normal: &Vec3,
    hit: &Vec3,
    recursive_depth: u8,
) -> Color {
    let reflection_direction = reflection_angle(direction, normal);
    let reflection_origin: Vec3;
    if reflection_direction.dot(*normal) < 0f32 {
        reflection_origin = hit - 0.0001f32;
    } else {
        reflection_origin = hit + 0.0001f32;
    }
    return cast_ray(
        scene,
        &reflection_origin,
        &reflection_direction,
        recursive_depth + 1,
    );
}

pub fn render(scene: Scene, output: &str) {
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
        *pixel = cast_ray(&scene, &Vec3::ZERO, &direction, 0).as_rgb();
    }

    frame_buffer.save(output).unwrap();
    println!("Image has been rendered and saved to {output}!");
}

fn reflection_angle(incident: &Vec3, normal: &Vec3) -> Vec3 {
    return incident - normal * 2f32 * incident.dot(*normal);
}

fn refraction_angle(incident: &Vec3, normal: &Vec3, refractive_index: f32) -> Vec3 {
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
