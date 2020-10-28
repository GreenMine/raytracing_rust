mod image_gen;
mod data_structures;
mod ray;

use ray::Ray;
use image_gen::PpmImage;
use data_structures::{Vec3, Color, Point3};
use std::{
    io::{
        self,
        prelude::*
    },
    fs::File
};


const IMAGE_WIDTH:  usize = 400;
const IMAGE_HEIGHT: usize = 400;

fn main() -> io::Result<()> {
    //Create image
    let mut stderr: io::Stderr = io::stderr();
    let mut image = PpmImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    //Render
    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanliner remaining {} ", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            image.write_vec3(Color((i as f64) / ((IMAGE_WIDTH  - 1) as f64),
                                        (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                                        0.25f64));
        }
    }

    //Save image
    write!(stderr, "\nSaving...")?;
    image.save_to_file(File::create("test_image.ppm")?)?;
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
