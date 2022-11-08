mod ray;
mod camera;
pub mod data_structures;

mod hittable;
mod hittable_list;
pub mod objects;

mod material;
pub mod materials;

pub use ray::Ray;
pub use camera::Camera;

pub use hittable::{HitInfo, Hittable};
pub use hittable_list::HittableList;

pub(self) use material::Material;

