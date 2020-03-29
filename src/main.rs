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
use std::sync::{Arc};

extern crate rand;
use rand::Rng;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(version = "1.0", author = "Joaquim Castilla")]
struct Opts {
    #[structopt(short = "o", long = "output", default_value = "output.ppm")]
    output: String,
    #[structopt(short = "x", long = "x_size", default_value = "250")]
    x_size: i32,
    #[structopt(short = "y", long = "y_size", default_value = "125")]
    y_size: i32,
}

fn color(ray: &Ray, world: &Vec<Sphere>, depth: i32) -> Vec3 {
    if let Some(record) = (world.hit(ray, 0.001, f32::MAX)) {
        if let Some(scatter_ret) = record.material.scatter(ray, &record) {
            if(depth < 50) {
                return scatter_ret.attenuation * color(&scatter_ret.ray, &world, depth + 1);
            }
            else {
                return Vec3::default();
            }
        }
        else {
            return Vec3::default();
        }
    }
    else {
        let unit_direction: Vec3 = unit_vector(ray.direction());
        let t: f32 = 0.5*(unit_direction.y() + 1.0);
        return (Vec3{ e: [1.0, 1.0, 1.0] }*(1.0 - t)) + (Vec3{ e: [0.5, 0.7, 1.0] }*t)
    }
}

fn random_scene() -> Vec<Sphere> {
    let n: i32 = 500;
    let mut world: Vec<Sphere> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut new_sphere = Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian { albedo: Vec3 {e: [0.5, 0.5, 0.5]} }));
    world.push(new_sphere);
    let i: i32 = 1;
    for a in -11 .. 11 {
        for b in -11 .. 11 {
            let choose_mat: f32 = rng.gen::<f32>();
            let center: Vec3 = Vec3::new(a as f32 +0.9*rng.gen::<f32>(), 0.2, b as f32 + 0.9*rng.gen::<f32>());
            if((center -  Vec3::new(4.0, 0.2, 0.0)).length() > 0.9) {
                if(choose_mat < 0.8) {
                    new_sphere = Sphere::new(center, 0.2,
                                            Arc::new(Lambertian { albedo: Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(),
                                                                                    rng.gen::<f32>() * rng.gen::<f32>(),
                                                                                    rng.gen::<f32>() * rng.gen::<f32>())}));
                    world.push(new_sphere);
                }
                else if(choose_mat < 0.95) {
                    new_sphere = Sphere::new(center, 0.2,
                                            Arc::new(Metal { albedo: Vec3::new(0.5 * (1.0 + rng.gen::<f32>()),
                                                                               0.5 * (1.0 + rng.gen::<f32>()),
                                                                               0.5 * (1.0 + rng.gen::<f32>())),
                                                                               fuzz: 0.5 * rng.gen::<f32>()}));
                    world.push(new_sphere);
                }
                else {
                    new_sphere = Sphere::new(center, 0.2,
                                            Arc::new(Dielectric { refraction_index: 1.5}));
                }
            }
        }
    }
    world.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric { refraction_index: 1.5})));
    world.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1)})));
    world.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0})));

    return world;
}

fn main() -> std::io::Result<()> {
    // Read input parameters
    let opts: Opts = Opts::from_args();

    let outfile = opts.output;
    let nx = opts.x_size;
    let ny = opts.y_size;
    let n_samples = 50;

    let mut file = File::create(outfile)?;
    let mut line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(line.as_bytes())?;

    let look_from: Vec3 = Vec3::new(9.0, 1.5, 3.0);
    let look_at: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let dist_to_focus: f32 = (look_from - look_at).length();
    let aperture: f32 = 0.0;
    let camera: Camera = Camera::new(look_from, look_at, Vec3::new(0.0, 1.0, 0.0), 30.0, nx as f32/ny as f32, aperture, dist_to_focus);
    let world = random_scene();
    // Random number generator
    let mut rng = rand::thread_rng();
    for j in (0 .. ny).rev() {
        for i in 0..nx {
            let mut col: Vec3 = Vec3::default();
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
