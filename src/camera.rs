use crate::vec3::*;
use crate::ray::Ray;
use crate::material::rand_point_in_unit_sphere;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfof: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let lens_radius: f32 = aperture/2.0;
        let theta: f32 = vfof * std::f32::consts::PI/180 as f32;
        let half_height: f32 = (theta/2.0).tan();
        let half_width: f32 = aspect * half_height;
        let mut u: Vec3;
        let mut v: Vec3;
        let mut w: Vec3;
        let origin: Vec3 = look_from;
        w = unit_vector(look_from - look_at);
        u = unit_vector(vup.cross(w));
        v = w.cross(u);
        let lower_left_corner: Vec3 = origin
                                    - u * half_width * focus_dist
                                    - v * half_height * focus_dist
                                    -  w * focus_dist;
        let horizontal: Vec3 =  u * 2.0 * half_width * focus_dist;
        let vertical: Vec3 = v * 2.0 * half_height * focus_dist;
        Camera {
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            origin: origin,
            u: u,
            v: v,
            w: w,
            lens_radius: lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd: Vec3 = rand_point_in_unit_sphere() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset
        }
    }
}