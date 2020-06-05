mod image_gen;

use image_gen::ppm_image::PpmImage;

use std::{
    io::{
        self,
        prelude::*
    },
    fs::File
};

const IMAGE_WIDTH:  usize = 1024;
const IMAGE_HEIGHT: usize = 1024;

fn main() -> io::Result<()> {
    let mut stderr: io::Stderr = io::stderr();
    let mut image = PpmImage::new(File::create("test_image.ppm")?, IMAGE_WIDTH, IMAGE_HEIGHT);
    image.open()?;

    for j in (0..IMAGE_HEIGHT).rev() {
        write!(stderr, "\rScanliner remaining {} ", j)?;
        stderr.flush()?;
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH  - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25f64;
            image.write_string(format!("{} {} {}\n", (255.999 * r) as i32, (255.999 * g) as i32, (255.999 * b) as i32))?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
