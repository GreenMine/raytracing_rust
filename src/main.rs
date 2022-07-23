mod image_gen;
mod ray_tracer;

use ray_tracer::{
    Hittable, HittableList, Ray,
    data_structures::{self, Vec3, Color, Point3, dot},
    objects::{Sphere, Triangle}
};
use image_gen::PpmImage;
use std::{
    io::{
        self,
        prelude::*
    },
    fs::File
};


const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH:  usize = 1920;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

fn main() -> io::Result<()> {
    //Create image
    let mut stderr: io::Stderr = io::stderr();
    let mut image = PpmImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    //Objects
    let mut world = HittableList::new();
    world.add(Sphere { center: Point3(0.0, 0.0, -1.0)   , radius: 0.5});
    world.add(Sphere { center: Point3(0.0, -100.5, -1.0), radius: 100.0});
    
    //Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0); //Max of horizontal position, i think
    let vertical = Vec3(0.0, viewport_height, 0.0); //Max of vertical position, i think
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length); //Get lower left cornet of the screen(horizontal and vertical divided by 2, because need half of the screen)

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanliner remaining {} ", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            image.write_vec3(ray_color(&ray));
        }
    }

    //Save image
    write!(stderr, "\nSaving...")?;
    image.save_to_file(File::create("test_image.ppm")?)?;
    write!(stderr, "\nDone.\n")?;
    Ok(())
}


fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3(0.0, 0.0, -1.0);


    let hit = hit_sphere(&sphere_center, 0.5, ray);
    if hit > 0.0 {
        let normals = data_structures::unit_vector(ray.at(hit) - sphere_center);
        return 0.5 * Color(normals.x() + 1.0, normals.y() + 1.0, normals.z() + 1.0);
    }
    let unit_direction = data_structures::unit_vector(ray.direction); //scaling to -1 < unit_direction < 1
    let t = 0.5 * (unit_direction.y() + 1.0); //scaling to 0 < t < 1
    (1.0 - t) * Color(1.0, 1.0, 1.0) + t * Color(0.5, 0.7, 1.0)
}