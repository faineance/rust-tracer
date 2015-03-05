extern crate image;
use camera::Camera;
use vector::Vector;
use std::fs::File;
use std::io::prelude::*;
mod sphere;
mod ray;
mod colour;
mod vector;
mod camera;

fn main() {
	let width = 640;
	let height = 480;
	let campos =  Vector::new(1.0,1.0,1.0);
	let camlook = Vector::empty();
	let camup = Vector::new(0.0, 1.0,0.0);
	let camera =  Camera::new(campos, camlook,camup, 45, width, height);

	let mut file = File::create("img.pbm");

	match  file {
		Ok(mut file) => {
			file.write("P3\n".as_bytes());
			file.write(format!("{} {} 255\n", camera.width, camera.height).as_bytes());
			for x in 0..width {
				for y in 0..height {
					let colour = camera.trace_ray(x, y);
					file.write(format!("{} {} {}\n", colour.red, colour.green, colour.blue).as_bytes());
				}
			}
		},

		Err(str) => {
			println!("{:?}", str);
		}
	}
	

}