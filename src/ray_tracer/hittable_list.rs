use super::Hittable;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

impl HittableList {

    pub fn new() -> Self {
        Self { objects: Vec::new()}
    }

    pub fn add<T: Hittable + 'static>(&mut self, value: T) {
        self.objects.push(Box::new(value));
    }
}