//Rust program to calculate the area of triangle given three sides

use std::io;

fn main() 
{
	let mut input_1 = String::new();
	let mut input_2 = String::new();
	let mut input_3 = String::new();

	println!("Enter first edge of triangle: ");
	io::stdin().read_line(&mut input_1).expect("Not a valid string");
 	let a:f32 = input_1.trim().parse().expect("Not a valid number");

 	println!("Enter second edge of triangle: ");
	io::stdin().read_line(&mut input_2).expect("Not a valid string");
 	let b:f32 = input_2.trim().parse().expect("Not a valid number");

 	println!("Enter third edge of triangle: ");
	io::stdin().read_line(&mut input_3).expect("Not a valid string");
 	let c:f32 = input_3.trim().parse().expect("Not a valid number");

 	let s:f32 = (a + b + c) / 2.0;
 	let mut area:f32 = s * (s - a) * (s - b) * (s - c);
 	area = area.sqrt();

 	println!("Area of a triangle: {}", area);
}
