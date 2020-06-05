use std::fs::File;
use std::io::prelude::*;
use crate::data_structures::Vec3;

pub struct PpmImage {
    pub(crate) file: File,
               width: usize,
               height: usize
}

impl PpmImage {
    pub fn new(file: File, width: usize, height: usize) -> PpmImage {
        PpmImage {file, width, height}
    }

    pub fn open(&mut self) -> Result<(), std::io::Error> {
        self.write_string(format!("P3\n{} {}\n255\n", self.width, self.height))
    }

    pub fn write_vec3(&mut self, vec: Vec3) -> Result<(), std::io::Error> {
        self.write_string(format!("{} {} {}\n", (vec.x() * 255.999) as u32, (vec.y() * 255.999) as u32, (vec.z() * 255.999) as u32))
    }

    pub fn write_string(&mut self, string: String) -> Result<(), std::io::Error> {
        self.file.write_all(string.as_bytes())
    }
}
