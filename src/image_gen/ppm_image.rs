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
        let mut buffer = vec![Vec::new(); height];
        Self {
            width,
            height,
            buffer,
        }
    }

    pub fn write_row(&mut self, index: usize, row: Vec<Color>) {
        self.buffer[index] = row;
    }

    pub fn save_to_file(self, mut file: File) -> Result<usize, std::io::Error> {
        file.write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;
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
