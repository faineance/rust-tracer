use vector::Vector;
use vector::{cross, dot};
#[derive(Debug,Copy)]
pub struct Camera {
	pub position: Vector,
	pub look_at: Vector,
	pub up: Vector,
	pub fov: f32,
	pub width: f32,
	pub height: f32,
	pub eye: Vector,
	pub right: Vector
}
impl Camera {
	pub fn new(position: Vector, look_at: Vector, up: Vector, fov: f32, width: f32, height: f32) -> Camera {
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
	}
}