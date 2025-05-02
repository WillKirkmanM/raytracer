mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::sync::Arc;
use std::time::Instant;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;

use vec3::{Color, Point3, Vec3};
use ray::Ray;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::zero();
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::zero();
    }

    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn create_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::rng();
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.random::<f64>();

            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(), 
                0.2, 
                b as f64 + 0.9 * rng.random::<f64>()
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random();
                    let material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, material.clone())));
                } else if choose_material < 0.95 {
                    let albedo = Color::random() * 0.5 + Color::new(0.5, 0.5, 0.5);
                    let fuzz = rng.random_range(0.0..0.5);
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let world = create_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio);

    println!("Rendering an image with dimensions: {}x{}", image_width, image_height);
    let start = Instant::now();
    
    let progress_bar = ProgressBar::new((image_height) as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let world_ref = Arc::new(world);

    img.enumerate_pixels_mut().collect::<Vec<_>>().par_iter_mut().for_each(|(x, y, pixel)| {
        let mut pixel_color = Color::zero();
        let mut rng = rand::rng();
        
        for _ in 0..samples_per_pixel {
            let u = ((*x as f64) + rng.random::<f64>()) / (image_width - 1) as f64;
            let v = ((image_height - 1 - *y) as f64 + rng.random::<f64>()) / (image_height - 1) as f64;
            
            let ray = camera.get_ray(u, v);
            pixel_color += ray_color(&ray, world_ref.as_ref(), max_depth);
        }
        
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (pixel_color.r() * scale).sqrt();
        let g = (pixel_color.g() * scale).sqrt();
        let b = (pixel_color.b() * scale).sqrt();
        
        **pixel = image::Rgb([
            (256.0 * r.clamp(0.0, 0.999)) as u8,
            (256.0 * g.clamp(0.0, 0.999)) as u8,
            (256.0 * b.clamp(0.0, 0.999)) as u8,
        ]);

    });

    progress_bar.finish_with_message("Rendering complete!");
    let duration = start.elapsed();

    println!("Saving image...");
    img.save("output.png").expect("Failed to save image");

    println!("Done in {:?}", duration);
    println!("Image saved to output.png");
}