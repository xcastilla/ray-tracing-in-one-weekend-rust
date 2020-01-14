use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: HitRecord) -> Option<HitRecord>;
}

pub trait HittableList: Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: HitRecord) -> Option<HitRecord>;
}
