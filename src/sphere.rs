use crate::vec3::Vec3;
use crate::hittable::*;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {center: center, radius: radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: HitRecord) -> Option<HitRecord> {
        let mut ret_rec: Option<HitRecord> = None;
        let oc: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().dot(ray.direction());
        let b: f32 = oc.dot(ray.direction());
        let c: f32 = oc.dot(oc) - self.radius * self.radius;
        let discriminant: f32 = b * b - a * c;
        if(discriminant > 0.0) {
            let mut temp: f32 = (-b - discriminant.sqrt())/a;
            if(temp < t_max && temp > t_min) {
                return Some(HitRecord{t: temp, p: ray.point_at_parameter(rec.t), normal: (rec.p - self.center)/self.radius })
            }
            temp = (-b + discriminant.sqrt())/a;
            if(temp < t_max && temp > t_min) {
                return Some(HitRecord{t: temp, p: ray.point_at_parameter(rec.t), normal: (rec.p - self.center)/self.radius })
            }
        }
        return ret_rec;
    }
}

impl Hittable for Vec<Sphere> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: HitRecord) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far: f32;
        for i in 0 .. self.len() {
            //let mut rec2 = rec.clone();
            if let Some(record) = self[i].hit(ray, t_min, t_max, rec) {
                closest_so_far = record.t;
                temp_rec = Some(record);
            }
        }
        return temp_rec;
    }
}
