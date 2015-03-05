use colour::Colour;
use sphere::Sphere;
use camera::Camera;
#[derive(Debug,Copy)]
pub struct Scene {
	pub background: Colour,
	pub objects: Vec<Sphere>,
	pub camera: Camera
}