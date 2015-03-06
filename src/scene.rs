use colour::Colour;
use sphere::Sphere;
use camera::Camera;
#[derive(Debug)]
pub struct Scene {
	pub background: Colour,
	pub objects: Vec<Sphere>,
	pub camera: Camera
}
impl Scene {
	pub fn new(background: Colour, objects: Vec<Sphere>, camera: Camera ) -> Scene {
		Scene { background: background, objects: objects, camera: camera }
	}

}