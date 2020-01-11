// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground
// Implements a ray with the form p(t) = a + b * t

use crate::vec3::*;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin: origin, direction: direction}
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    // Evaluate at t
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
