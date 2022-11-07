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
use std::sync::{Arc, Mutex};
use std::{
    fs::File,
    io::{self, prelude::*},
};

use rayon::prelude::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 3840;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: u16 = 100;

const MAX_DEPTH: usize = 50;

fn main() -> io::Result<()> {
    //Create image
    let image = PpmImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    let image = Arc::new(Mutex::new(image));

    //Objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3(0.0, -100.5, -1.0), 100.0));

    let world = Arc::new(world);

    //Camera
    let camera = Camera::new();

    //Render
    (0..IMAGE_HEIGHT).into_par_iter().rev().for_each(|j| {
        let w = world.clone();
        let image = image.clone();
        let mut rand = rand::thread_rng();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(ray, &w, MAX_DEPTH);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;

            image
                .lock()
                .unwrap()
                .write_vec3(pixel_color, SAMPLES_PER_PIXEL);
        }
    });

    //Save image
    let mut stderr: io::Stderr = io::stderr();
    write!(stderr, "\nSaving...")?;
    image
        .lock()
        .unwrap()
        .save_to_file(File::create("result/image.ppm")?)?;
    write!(stderr, "\nDone.\n")?;
    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList, depth: usize) -> Color {
    let mut hit_info = HitInfo::default();

    if depth <= 0 {
        return Color(0.0, 0.0, 0.0);
    }

    if world.hit(&ray, 0.001, f64::INFINITY, &mut hit_info) {
        let target = hit_info.point + hit_info.normal + Vec3::random_unit_sphere();
        return 0.5
            * ray_color(
                Ray::new(hit_info.point, target - hit_info.point),
                world,
                depth - 1,
            );
    }

    let unit_direction = unit_vector(ray.direction); //scaling to -1 < unit_direction < 1
    let t = 0.5 * (unit_direction.y() + 1.0); //scaling to 0 < t < 1

    //blendedValue = (1−t) * startValue + t * endValue
    //Из формулы описанной выше, мы получаем что мы хотим получить линейный градиент
    //Из белого в голубой
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}
