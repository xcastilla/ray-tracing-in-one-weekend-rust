use crate::vec3::*;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera{
        Camera {
            lower_left_corner: Vec3 { e: [-2.0, -1.0, -1.0] },
            horizontal: Vec3 { e: [4.0, 0.0, 0.0] },
            vertical: Vec3 { e: [0.0, 2.0, 0.0] },
            origin: Vec3 { e: [0.0, 0.0, 0.0] }
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
        }
    }
}