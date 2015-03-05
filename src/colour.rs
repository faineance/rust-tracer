use std::num::Float;
#[derive(Debug, Copy)] 
pub struct Colour {
	pub red: u8,
	pub green: u8,
	pub blue: u8
}
impl Colour {
	pub fn new(red: u8, green: u8, blue: u8 ) -> Colour {
		Colour { red: red, green: green, blue: blue }
	}
}
pub fn clamp(x: f64) -> f64
{
	if x < 0.0 {
		return 0.0;
	}
	if x > 1.0 {
		return 1.0;
	}
	x
}
pub fn to_int(x: f64) -> i64
{
	(clamp(x).powf(1.0 / 2.2) * 255.0 + 0.5) as i64
}