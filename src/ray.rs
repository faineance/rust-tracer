use vector::Vector;
#[derive(Debug, Copy)]
pub struct Ray {
	pub origin: Vector,
	pub direction: Vector
}

impl Ray {
	pub fn new(origin: Vector, direction: Vector ) -> Ray {
		Ray { origin: origin, direction: direction }
	}
}