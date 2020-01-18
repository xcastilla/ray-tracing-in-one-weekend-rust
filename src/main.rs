mod vec3;
mod ray;
mod hittable;
mod sphere;
mod camera;
mod material;

use std::f32;
use std::fs::File;
use std::io::prelude::*;
use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;
use camera::Camera;
use material::*;
use std::sync::Arc;

extern crate rand;
use rand::Rng;

fn color(ray: &Ray, world: &Vec<Sphere>, depth: i32) -> Vec3 {
    if let Some(record) = (world.hit(ray, 0.001, f32::MAX)) {
        if let Some(scatter_ret) = record.material.scatter(ray, &record) {
            if(depth < 50) {
                return scatter_ret.attenuation * color(&scatter_ret.ray, &world, depth + 1);
            }
            else {
                return Vec3 { e: [0.0, 0.0, 0.0] }
            }
        }
        else {
            return Vec3 { e: [0.0, 0.0, 0.0] }
        }
    }
    else {
        let unit_direction: Vec3 = unit_vector(ray.direction());
        let t: f32 = 0.5*(unit_direction.y() + 1.0);
        return (Vec3{ e: [1.0, 1.0, 1.0] }*(1.0 - t)) + (Vec3{ e: [0.5, 0.7, 1.0] }*t)
    }
}


fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;
    let n_samples = 100;

    let mut file = File::create("foo.ppm")?;
    let mut line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(line.as_bytes())?;

    let camera: Camera = Camera::new();
    let world: Vec<Sphere> = vec![Sphere{ center: Vec3{e: [0.0, 0.0, -1.0]}, radius: 0.5, material: Arc::new(Lambertian { albedo: Vec3 {e: [0.1, 0.2, 0.5]} })},
                                  Sphere{ center: Vec3{e: [0.0, -100.5, -1.0]}, radius: 100.0, material: Arc::new(Lambertian { albedo: Vec3 { e: [0.8, 0.8, 0.0]} })},
                                  Sphere{ center: Vec3{e: [1.0, 0.0, -1.0]}, radius: 0.5, material: Arc::new(Metal { albedo: Vec3 { e: [0.8, 0.6, 0.2]}, fuzz: 0.3 })},
                                  Sphere{ center: Vec3{e: [-1.0, 0.0, -1.0]}, radius: 0.5, material: Arc::new(Dielectric { refraction_index: 1.5 }) },
                                  Sphere{ center: Vec3{e: [-1.0, 0.0, -1.0]}, radius: -0.45, material: Arc::new(Dielectric { refraction_index: 1.5 }) } ] ;
    // Random number generator
    let mut rng = rand::thread_rng();

    for j in (0 .. ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Vec3 { e: [0.0, 0.0, 0.0] };
            for s in 0..n_samples {
                let u: f32 = (i as f32 + rng.gen::<f32>()) /nx as f32;
                let v: f32 = (j as f32 + rng.gen::<f32>())/ny as f32;
                let ray: Ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= n_samples as f32;
            col = Vec3 { e: [col.x().sqrt(), col.y().sqrt(), col.z().sqrt()] };
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            line = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(line.as_bytes())?;
        }
       
    }
    Ok(())
}
