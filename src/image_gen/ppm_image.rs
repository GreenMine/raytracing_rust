use std::fs::File;
use std::io::prelude::*;
use crate::data_structures::Vec3;

pub struct PpmImage {
   width: usize,
   height: usize,
   buffer: String
}

impl PpmImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width, height, buffer: format!("P3\n{} {}\n255\n", width, height)}
    }

    pub fn write_vec3(&mut self, vec: Vec3) {
        self.write_string(format!("{} {} {}\n", (vec.x() * 255.999) as u32, (vec.y() * 255.999) as u32, (vec.z() * 255.999) as u32))
    }

    pub fn write_string(&mut self, string: String) {
        self.buffer += &string[..];
    }
    
    pub fn save_to_file(&mut self, mut file: File) -> Result<usize, std::io::Error> {
        file.write(self.buffer.as_bytes())
    }
}
