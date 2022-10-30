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
use std::{
    fs::File,
    io::{self, prelude::*},
};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 1920;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: u16 = 100;

fn main() -> io::Result<()> {
    //Create image
    let mut stderr: io::Stderr = io::stderr();
    let mut image = PpmImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    //Objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3(0.0, -101.0, -1.0), 100.0));

    //Camera
    let camera = Camera::new();

    //Random
    let mut rand = rand::thread_rng();

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanlines remaining: {}", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(ray, &world);
            }

            image.write_vec3(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    //Save image
    write!(stderr, "\nSaving...")?;
    image.save_to_file(File::create("result/image.ppm")?)?;
    write!(stderr, "\nDone.\n")?;
    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList) -> Color {
    let mut hit_info = HitInfo::default();
    if world.hit(&ray, 0.0, f64::MAX, &mut hit_info) {
        return 0.5 * (hit_info.normal + Color(1.0, 1.0, 1.0));
    }

    let unit_direction = unit_vector(ray.direction); //scaling to -1 < unit_direction < 1
    let t = 0.5 * (unit_direction.y() + 1.0); //scaling to 0 < t < 1

    //blendedValue = (1−t) * startValue + t * endValue
    //Из формулы описанной выше, мы получаем что мы хотим получить линейный градиент
    //Из белого в голубой
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}
