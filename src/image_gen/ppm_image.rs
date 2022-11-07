use crate::data_structures::Vec3;
use crate::ray_tracer::data_structures::Color;
use std::fs::File;
use std::io::prelude::*;

pub struct PpmImage {
    width: usize,
    height: usize,
    buffer: Vec<Vec<Vec3>>,
}

impl PpmImage {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Vec::new(); height];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn write_row(&mut self, index: usize, row: Vec<Color>) {
        self.buffer[index] = row;
    }

    pub fn save_to_file(self, mut file: File) -> Result<(), std::io::Error> {
        fn calculate_vec3(string: &mut String, vec: Vec3) {
            *string += &format!(
                "{} {} {}\n",
                (256.0 * clamp(vec.x().sqrt(), 0.0, 0.999)) as u32,
                (256.0 * clamp(vec.y().sqrt(), 0.0, 0.999)) as u32,
                (256.0 * clamp(vec.z().sqrt(), 0.0, 0.999)) as u32
            );
        }

        let mut buffer = String::with_capacity(self.width * self.height * 32);
        buffer += &format!("P3\n{} {}\n255\n", self.width, self.height);

        for rows in self.buffer {
            for column in rows {
                calculate_vec3(&mut buffer, column);
            }
        }

        file.write(buffer.as_bytes())?;
        Ok(())
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
