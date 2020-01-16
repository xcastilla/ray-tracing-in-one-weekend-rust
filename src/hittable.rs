use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::*;

//#[derive(Copy, Clone)]
pub struct HitRecord <'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub trait HittableList: Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
