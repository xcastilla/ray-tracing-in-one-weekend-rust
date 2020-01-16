use crate::vec3::*;
use crate::hittable::*;
use crate::ray::Ray;
use crate::material::*;
use std::sync::Arc;

pub struct Sphere{
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Sphere {
        Sphere {center: center, radius: radius, material: material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut ret_rec: Option<HitRecord> = None;
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().dot(ray.direction());
        let b: f32 = oc.dot(ray.direction());
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if(discriminant > 0.0) {
            let mut temp: f32 = (-b - discriminant.sqrt())/a;
            if(temp < t_max && temp > t_min) {
                let p: Vec3 = ray.point_at_parameter(temp);
                let normal: Vec3 = unit_vector((p - self.center)/self.radius);
                //let material: Box<Material> = Box::new(*self.material);
                return Some(HitRecord{t: temp, p: p, normal: normal, material: &*self.material })
            }
            temp = (-b + discriminant.sqrt())/a;
            if(temp < t_max && temp > t_min) {
                let p: Vec3 = ray.point_at_parameter(temp);
                let normal: Vec3 = unit_vector((p - self.center)/self.radius);
                //let material: Box<Material> = Box::new(*self.material);
                return Some(HitRecord{t: temp, p: p, normal: normal, material: &*self.material })
            }
        }
        return ret_rec;
    }
}

impl Hittable for Vec<Sphere> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far: f32 = t_max;
        for i in 0 .. self.len() {
            if let Some(record) = self[i].hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                temp_rec = Some(record);
            }
        }
        return temp_rec;
    }
}
