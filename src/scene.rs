use colour::Colour;
use sphere::Sphere;
use camera::Camera;
use std::fs::File;
use std::io::prelude::*;
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
	pub fn trace(self) {
		let mut output = File::create("output.pbm");

		match  output {
			Ok(mut output) => {
				output.write("P3\n".as_bytes());
				output.write(format!("{} {} 255\n", self.camera.width, self.camera.height).as_bytes());
				for x in 0..self.camera.width {
					for y in 0..self.camera.height {
						
						let colour = self.camera.trace_ray(x, y);
						output.write(format!("{} {} {}\n", colour.red, colour.green, colour.blue).as_bytes());
					}
				}

			},

			Err(str) => {
				println!("{:?}", str);
			}
		}
	}

}