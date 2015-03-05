use vector::Vector;
use colour::Colour;
use vector::{cross, dot};
#[derive(Debug,Copy)]
pub struct Camera {
	pub position: Vector,
	pub look_at: Vector,
	pub up: Vector,
	pub fov: u32,
	pub width: u32,
	pub height: u32,
	pub eye: Vector,
	pub right: Vector
}
impl Camera {
	pub fn new(position: Vector, look_at: Vector, up: Vector, fov: u32, width: u32, height: u32) -> Camera {
		let mut camera = Camera {
			position: position,
			look_at: look_at,
			up: up,
			fov: fov,
			width: width,
			height: height,
			eye: Vector::empty(),
			right: Vector::empty()
		};
		camera.eye = (camera.look_at - camera.position).normalize();
		camera.right = cross(camera.eye, camera.up);
		camera
	}

	pub fn trace_ray(self,x : u32, y : u32) -> Colour {
		Colour::new(255,0,0)
	}
}