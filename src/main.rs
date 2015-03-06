#![feature(fs,io)]
use camera::Camera;
use sphere::Sphere;
use colour::Colour;
use scene::Scene;
use vector::Vector;
use std::fs::File;
use std::io::prelude::*;
mod sphere;
mod ray;
mod colour;
mod vector;
mod camera;
mod scene;
fn main() {
	let width = 640;
	let height = 480;

	let campos =  Vector::new(1.0,1.0,1.0);
	let camlook = Vector::empty(); //center
	let camup = Vector::new(0.0, 1.0,0.0);
	let camera =  Camera::new(campos, camlook,camup, 45, width, height);

	let objects = vec![Sphere::new(0.5, Vector::empty(), Colour::new(0,0,0))];
	let scene =  Scene::new(Colour::new(255,255,255), objects, camera);
	let mut output = File::create("output.pbm");

	match  output {
		Ok(mut output) => {
			output.write("P3\n".as_bytes());
			output.write(format!("{} {} 255\n", camera.width, camera.height).as_bytes());
			for x in 0..width {
				for y in 0..height {
					let colour = camera.trace_ray(x, y);
					output.write(format!("{} {} {}\n", colour.red, colour.green, colour.blue).as_bytes());
				}
			}

		},

		Err(str) => {
			println!("{:?}", str);
		}
	}
	

}