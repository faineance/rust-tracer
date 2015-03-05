use std::ops::{Add, Sub, Mul};
use std::num::Float;
#[derive(Debug,Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
    //Calculating the squared length of a vector is faster than calculating its length.
    //For use when determining whether one vector is longer then another.
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    //Outputs the length of the input value. 
    pub fn length(&self) -> f32 {
        Float::sqrt(self.length_squared())
    }
    //Outputs a new unit vector with the same direction and orientation as the input vector. 
    pub fn normalize(&self) -> Vector {
       let length = self.length();
        Vector { x: self.x / length, y: self.y / length, z: self.z / length }  
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}
pub fn cross(a: Vector, b: Vector) -> Vector {
    Vector {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x
    }
}
pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}
