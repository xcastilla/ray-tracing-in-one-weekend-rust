mod vec3;
mod ray;

use std::fs::File;
use std::io::prelude::*;
use vec3::*;
use ray::*;

fn color(ray: &Ray) -> Vec3 {
    let t: f32 = hit_sphere(&Vec3 { e: [0.0, 0.0, -1.0] }, 0.5, &ray);
    if(t > 0.0) {
        let N: Vec3 = unit_vector(ray.point_at_parameter(t) - Vec3 { e: [0.0, 0.0, -1.0]});
        return Vec3{ e: [N.x() + 1.0, N.y() + 1.0, N.z() + 1.0] } * 0.5;
    }
    let unit_direction: Vec3 = unit_vector(ray.direction());
    let t: f32 = 0.5*(unit_direction.y() + 1.0);
    (Vec3{ e: [1.0, 1.0, 1.0] }*(1.0 - t)) + (Vec3{ e: [0.5, 0.7, 1.0] }*t)
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc: Vec3 = ray.origin() - * center;
    let a: f32 = ray.direction().dot(ray.direction());
    let b: f32 = 2.0 * oc.dot(ray.direction());
    let c: f32 = oc.dot(oc) - radius * radius;
    let discriminant: f32 = b * b - 4.0 * a * c;
    if(discriminant < 0.0) {
        return -1.0
    }
    else {
        return (-b - discriminant.sqrt())/(2.0*a);
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

    for j in (0 .. ny).rev() {
        for i in 0..nx {
            let u: f32 = i as f32/nx as f32;
            let v: f32 = j as f32/ny as f32;
            let ray: Ray = Ray{ origin: origin, direction: (lower_left_corner + (horizontal * u) + (vertical * v)) };
            let col: Vec3 = color(&ray);
            let ir: u32 = (255.99*col[0]) as u32;
            let ig: u32 = (255.99*col[1]) as u32;
            let ib: u32 = (255.99*col[2]) as u32;
            line = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(line.as_bytes())?;
        }
       
    }
    Ok(())
}
