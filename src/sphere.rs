use vector::Vector;
use colour::Colour;
use vector::{dot, cross};
use ray::Ray;
use std::num::Float;


#[derive(Debug,Copy)]
enum Reflection {
    Diffuse,
    Specular,
    Refractive
}
#[derive(Debug,Copy)]
pub struct Sphere {
    pub radius: f32,
    pub position: Vector,
    pub colour: Colour,
    // pub emission: Vector,
    // pub reflection: Reflection
} 
impl Sphere {
    pub fn new(radius: f32, position: Vector, colour: Colour) -> Sphere {
        Sphere { radius: radius, position: position, colour: colour}
    }
    //Returns distance if hit, returns 0 if no hit.
    pub fn intersect(&self, ray: Ray) -> f32 {
        let op = self.position - ray.origin; 
        let eps = 1e-4;
        let b = dot(op ,ray.direction);
        let mut det = b * b - dot(op ,ray.direction) + self.radius * self.radius;

        if det < 0.0 {
            return 0.0;
        } else {
            det = det.sqrt();
        }
        if (b - det) > eps {
            return b-det;
        }
        if (b + det) > eps {
            return b+det;
        }
        return 0.0;

    }
}
