use crate::vec3::{Vec3, unit_vector};
use crate::ray::Ray;
use crate::hittable::*;
extern crate rand;
use rand::Rng;
use std::sync::Arc;

pub trait Material{
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRet>;
}

pub struct ScatterRet {
    pub attenuation: Vec3,
    pub ray: Ray,
}

pub struct Lambertian {
    pub albedo: Vec3,
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

pub struct Dielectric {
    pub refraction_index: f32,
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv: Vec3 = unit_vector(v);
    let dt: f32 = uv.dot(n);
    let discriminant: f32 = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);
    if(discriminant > 0.0) {
        return Some((uv - n*dt)*ni_over_nt - n*discriminant.sqrt());
    }
    return None;
}


// Util function for Lambertian surface scattering implementation
fn rand_point_in_unit_sphere() -> Vec3 {
    let mut p: Vec3;
    let mut rng = rand::thread_rng();
    loop {
        p = Vec3{ e: [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()] }*2.0 - Vec3{ e:[1.0, 1.0, 1.0] };
        if(p.squared_length() < 1.0) {
            break;
        }
    }
    return p;
}

// Scattering function implementations for each material

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRet> {
        let target: Vec3 = hit_record.p + hit_record.normal + rand_point_in_unit_sphere();
        Some(ScatterRet{ attenuation: self.albedo, ray: Ray{ origin: hit_record.p, direction: target - hit_record.p }})
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRet> {
        let reflected: Vec3 = reflect(unit_vector(ray_in.direction), hit_record.normal);
        let scattered: Ray = Ray { origin: hit_record.p, direction: reflected + rand_point_in_unit_sphere()*self.fuzz };
        let attenuation = self.albedo;
        Some(ScatterRet{ attenuation: attenuation, ray: scattered})
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRet> {
        let mut outward_normal: Vec3;
        let reflected: Vec3 = reflect(ray_in.direction, hit_record.normal);
        let attenuation: Vec3 = Vec3 { e: [1.0, 1.0, 1.0] };
        let mut ni_over_nt: f32;
        if(ray_in.direction.dot(hit_record.normal) > 0.0) {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.refraction_index;
        }
        else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0/self.refraction_index;
        }

        if let Some(refracted) = refract(ray_in.direction, outward_normal, ni_over_nt) {
            let scattered: Ray = Ray{ origin: hit_record.p, direction: refracted }; 
            return Some(ScatterRet{ attenuation: attenuation, ray: scattered})
        }
        else {
            let scattered: Ray = Ray{ origin: hit_record.p, direction: reflected }; 
            return Some(ScatterRet{ attenuation: attenuation, ray: scattered});
        }

    }
}

impl Material for Arc<Material> {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRet> {
        (*self).scatter(&ray_in, &hit_record)
    }
}



