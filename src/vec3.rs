// Based on https://raytracing.github.io/books/RayTracingInOneWeekend.html#thevec3class

use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign, Index, IndexMut};
#[derive(Copy, Clone)]

pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2],
        }
    }

    // X Y Z getters
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }   

    // R G B getters
    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }  

    // Convert Vector into unit vector
    pub fn make_unit_vector(&mut self) {
        let k: f32 = 1.0 / (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt();
        *self = Vec3 { e: [self.e[0]/k , self.e[1]/k, self.e[2]/k] }
    }
    
    // Dot and cross products with other vec3
    
    pub fn dot(&self, other: Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {e: [self.e[1] * other.e[2] - self.e[2] * other.e[1],
                  self.e[2] * other.e[0] - self.e[0] * other.e[2],
                  self.e[0] * other.e[1] - self.e[1] * other.e[0]]}
    }
}

// Vector +, -, *, / and unary negation (-) operator overloads
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

// Multiplication by vec3
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

// Multiplication by float
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

// Division by Vec3
impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]] }
    }
}

// Division by float
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

// Vector +=, -=, *=, /=  operator overloads
impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] } 
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

impl DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Vec3 { e: [self.e[0] / other.e[0], self.e[1] / other.e[1], self.e[2] / other.e[2]] }
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self = Vec3 { e: [self.e[0] / t, self.e[1] / t, self.e[2] / t] }
    }
}

// Random access operator
impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &f32 {
        &self.e[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut f32 {
        &mut self.e[idx]
    }
}

// Get unit vector out of some Vec3
pub fn unit_vector(v: Vec3) -> Vec3 {
    v/v.length()
}
