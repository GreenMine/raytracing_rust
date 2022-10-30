use crate::ray_tracer::data_structures::Point3;
use crate::ray_tracer::Ray;

pub struct Camera {
    origin: Point3,            //Позиция камеры
    horizontal: Point3,        //Максимальная горизонтальная(по x) позиция
    vertical: Point3,          //Максимальная вертикальная(по y) позиция
    lower_left_corner: Point3, //левый нижний угол на 3-х мерном графике
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;
impl Camera {
    pub fn new() -> Self {
        Self::new_origin(Point3(0.0, 0.0, 0.0))
    }

    pub fn new_origin(origin: Point3) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ASPECT_RATIO;
        let focal_length = 1.0;

        let horizontal = Point3(viewport_width, 0.0, 0.0);
        let vertical = Point3(0.0, viewport_height, 0.0);

        //Деление на 2 необходимо, т.к. на графике у нас есть положительные и отрицательные части
        //origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length); => аналогия
        let lower_left_corner =
            origin - Point3(viewport_width / 2.0, viewport_height / 2.0, focal_length);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
