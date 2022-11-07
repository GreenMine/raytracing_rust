use crate::data_structures::Vec3;
use std::fs::File;
use std::io::prelude::*;

pub struct PpmImage {
    _width: usize,
    _height: usize,
    buffer: String,
}

impl PpmImage {
    // :o
    pub fn new(width: usize, height: usize) -> Self {
        let mut buffer = String::with_capacity(width * height * 100);
        buffer += &format!("P3\n{} {}\n255\n", width, height);
        Self {
            _width: width,
            _height: height,
            buffer,
        }
    }

    pub fn write_vec3(&mut self, mut vec: Vec3, samples_per_pixel: u16) {
        vec /= samples_per_pixel as f64;

        self.write_string(format!(
            "{} {} {}\n",
            (256.0 * clamp(vec.x().sqrt(), 0.0, 0.999)) as u32,
            (256.0 * clamp(vec.y().sqrt(), 0.0, 0.999)) as u32,
            (256.0 * clamp(vec.z().sqrt(), 0.0, 0.999)) as u32
        ))
    }

    pub fn write_string(&mut self, string: String) {
        self.buffer += &string[..];
    }

    pub fn save_to_file(&mut self, mut file: File) -> Result<usize, std::io::Error> {
        file.write(self.buffer.as_bytes())
    }
}

const fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
