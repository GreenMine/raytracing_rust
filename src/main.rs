mod image_gen;
mod ray_tracer;

use crate::data_structures::unit_vector;
use crate::ray_tracer::HitInfo;
use image_gen::PpmImage;
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

fn main() -> io::Result<()> {
    //Create image
    let mut stderr: io::Stderr = io::stderr();
    let mut image = PpmImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    //Objects
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3(0.0, 0.0, -1.0), 0.5));

    //Camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0); //Максимальная горизонтальная(по x) позиция
    let vertical = Vec3(0.0, viewport_height, 0.0); //Максимальная вертикальная(по y) позиция

    //Получаем левый нижний угол на 3-х мерном графике
    //Деление на 2 необходимо, т.к. на графике у нас есть положительные и отрицательные части
    //origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length); => аналогия
    let lower_left_corner =
        origin - Vec3(viewport_width / 2.0, viewport_height / 2.0, focal_length);

    println!("Horizontal: {}", horizontal);
    println!("Vertical: {}", vertical);
    println!("Left corner: {}", lower_left_corner);

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanlines remaining: {}", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            image.write_vec3(ray_color(ray, &mut world));
        }
    }

    //Save image
    write!(stderr, "\nSaving...")?;
    image.save_to_file(File::create("test_image.ppm")?)?;
    write!(stderr, "\nDone.\n")?;
    Ok(())
}

fn ray_color(ray: Ray, world: &mut HittableList) -> Color {
    let unit_direction = unit_vector(ray.direction); //scaling to -1 < unit_direction < 1
    let t = 0.5 * (unit_direction.y() + 1.0); //scaling to 0 < t < 1

    let mut hit_info = HitInfo::default();
    let hit = world.hit(&ray, 0.0, f64::MAX, &mut hit_info);

    if hit {
        let n = unit_vector(hit_info.point - Point3(0.0, 0.0, 1.0));
        return 0.5 * Color(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    //blendedValue = (1−t) * startValue + t * endValue
    //Из формулы описанной выше, мы получаем что мы хотим получить линейный градиент
    //Из белого в голубой
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}
