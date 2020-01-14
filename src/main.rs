mod vec3;
mod ray;
mod hittable;
mod sphere;

use std::fs::File;
use std::io::prelude::*;
use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;
use std::f32;


fn color(ray: &Ray, world: &Vec<Sphere>) -> Vec3 {
    let rec: HitRecord = HitRecord{ t: 0.0, p: Vec3{e: [0.0, 0.0, 0.0]}, normal: Vec3{ e: [0.0, 0.0, 0.0]} };
    if let Some(record) = (world.hit(ray, 0.0, f32::MAX, rec)) {
        let N = record.normal;
            return Vec3{ e: [N.x() + 1.0, N.y() + 1.0, N.z() + 1.0] } * 0.5;
    }
    else {
        let unit_direction: Vec3 = unit_vector(ray.direction());
        let t: f32 = 0.5*(unit_direction.y() + 1.0);
        (Vec3{ e: [1.0, 1.0, 1.0] }*(1.0 - t)) + (Vec3{ e: [0.5, 0.7, 1.0] }*t)
    }
}


fn main() -> std::io::Result<()> {
    let nx = 200;
    let ny = 100;

    let mut file = File::create("foo.ppm")?;
    let mut line = format!("P3\n{} {}\n255\n", nx, ny);
    file.write_all(line.as_bytes())?;

    let lower_left_corner: Vec3 = Vec3{ e: [-2.0, -1.0, -1.0] };
    let horizontal: Vec3 = Vec3{ e: [4.0, 0.0, 0.0] };
    let vertical: Vec3 = Vec3{ e: [0.0, 2.0, 0.0] };
    let origin: Vec3 = Vec3{ e: [0.0, 0.0, 0.0] };

    let world: Vec<Sphere> = vec![Sphere{ center: Vec3{e: [0.0, 0.0, -1.0]}, radius: 0.5 }, Sphere{ center: Vec3{e: [0.0, -100.5, -1.0]}, radius: 100.0 }];

    for j in (0 .. ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32/nx as f32;
            let v: f32 = j as f32/ny as f32;
            let ray: Ray = Ray{ origin: origin, direction: (lower_left_corner + (horizontal * u) + (vertical * v)) };
            let p = ray.point_at_parameter(2.0);
            let col: Vec3 = color(&ray, &world);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            line = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(line.as_bytes())?;
        }
       
    }
    Ok(())
}
