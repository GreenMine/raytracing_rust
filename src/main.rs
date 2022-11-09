#![feature(const_fn_floating_point_arithmetic)]

mod image_gen;
mod ray_tracer;

use crate::data_structures::unit_vector;
use crate::ray_tracer::{Camera, HitInfo};
use image_gen::PpmImage;
use rand::Rng;
use ray_tracer::{
    data_structures::{self, Color, Point3, Vec3},
    objects::Sphere,
    HittableList, Ray,
};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::{fs::File, io};

use crate::ray_tracer::materials::Lambertian;
use rayon::prelude::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: u16 = 100;

const MAX_DEPTH: usize = 50;

fn main() -> io::Result<()> {
    //Objects
    let mut world = HittableList::new();
    world.add(Sphere::new(
        Point3(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color(0.8, 0.8, 0.0))),
    ));
    world.add(Sphere::new(
        Point3(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color(0.7, 0.3, 0.3))),
    ));

    //Camera
    let camera = Camera::new();

    //Render
    let scanlines_left: AtomicUsize = AtomicUsize::new(IMAGE_HEIGHT);
    let image = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(|j| {
            let remaining = scanlines_left.fetch_sub(1, Relaxed);
            print!("\rScanlines remaining: {}", remaining);

            let mut rand = rand::thread_rng();
            (0..IMAGE_WIDTH)
                .map(|i| {
                    (0..SAMPLES_PER_PIXEL).fold(Color::default(), |fold, _| {
                        let u = (i as f64 + rand.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + rand.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                        let ray = camera.get_ray(u, v);

                        fold + ray_color(ray, &world, MAX_DEPTH)
                    }) / (SAMPLES_PER_PIXEL as f64)
                })
                .collect::<Vec<_>>()
        })
        .collect::<PpmImage>();

    //Save image
    println!("\nSaving...");
    image.save_to_file(File::create("result/image.ppm")?)?;
    println!("Done");
    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList, depth: usize) -> Color {
    let mut hit_info = HitInfo::default();

    if depth == 0 {
        return Color::default();
    }

    let hit = { world.hit(&ray, 0.001, f64::INFINITY, &mut hit_info) };
    if hit {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if hit_info.material.as_ref().unwrap().scatter(
            &ray,
            &hit_info,
            &mut attenuation,
            &mut scattered,
        ) {
            return attenuation * ray_color(scattered, &world, depth - 1);
        }
        return Color::default();
    }

    let unit_direction = unit_vector(ray.direction); //scaling to -1 < unit_direction < 1
    let t = 0.5 * (unit_direction.y() + 1.0); //scaling to 0 < t < 1

    //blendedValue = (1−t) * startValue + t * endValue
    //Из формулы описанной выше, мы получаем что мы хотим получить линейный градиент
    //Из белого в голубой
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}
