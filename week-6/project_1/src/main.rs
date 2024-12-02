use std::io;

fn main() {
    // defining the array with the list of formula names 
    let name_arr = ["Area of Trapezium","Area of Rhombus","Area of Parallelogram","Area of Cube","Volume of Cylinder"];
    let formula_arr = ["height/2*(base1+base2)","1/2*diagonal1*diagonal2","base*altitude","6*(length of the side)^2","pi*(radius^2)*height"];

    for index in 0..5 {
		println!("\nThe index number {} represents : {}", index,name_arr[index]); 
		println!("The formula for {} is : {}", name_arr[index],formula_arr[index]);
	}

    user_input();
}

fn user_input() {
	println!("\nWhich formula do you want?(Enter 0,1,2,3 or 4):");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let user_input:i32 = input.trim().parse().expect("Invalid input");

	if user_input == 0 {     // condition to find the area of trapezium
		println!("Formula for Area of Trapezium = (height/2.0) * (base1 + base2)");
		println!("Enter a value for height(in cm): ");
		let mut input1 = String::new();
		io::stdin().read_line(&mut input1).expect("Failed to read input");
		let height:f32 = input1.trim().parse().expect("Invalid input");

		println!("Enter a value for base1(in cm): ");
		let mut input2 = String::new();
		io::stdin().read_line(&mut input2).expect("Failed to read input");
		let base1:f32 = input2.trim().parse().expect("Invalid input");

		println!("Enter a value for base2(in cm): ");
		let mut input3 = String::new();
		io::stdin().read_line(&mut input3).expect("Failed to read input");
		let base2:f32 = input3.trim().parse().expect("Invalid input");

		let area_trapezium:f32 = (height/2.0)*(base1 + base2);
		println!("The Area of a Trapezium with 'height' = {}cm, 'base1' = {}cm and 'base2' = {}cm is: {}cm^2", height,base1,base2,area_trapezium);
	}
	else if user_input == 1 {     // condition to find the area of rhombus
		println!("Formula for Area of Rhombus = 1/2 * diagonal1 * diagonal2");
		println!("Enter a value for diagonal1(in cm): ");
		let mut input4 = String::new();
		io::stdin().read_line(&mut input4).expect("Failed to read input");
		let diagonal1:f32 = input4.trim().parse().expect("Invalid input");

		println!("Enter a value for diagonal2(in cm): ");
		let mut input5 = String::new();
		io::stdin().read_line(&mut input5).expect("Failed to read input");
		let diagonal2:f32 = input5.trim().parse().expect("Invalid input");

		let area_rhombus:f32 = (1.0/2.0)*diagonal1*diagonal2;
		println!("The Area of a Rhombus with 'diagonal1' = {}cm and 'diagonal2' = {}cm is: {}cm^2", diagonal1,diagonal2,area_rhombus);
	}
	else if user_input == 2 {     // condition to find the area of parallelogram
		println!("Formula for Area of Parallelogram = base * altitude");
		println!("Enter a value for base(in cm): ");
		let mut input6 = String::new();
		io::stdin().read_line(&mut input6).expect("Failed to read input"); 
		let base:f32 = input6.trim().parse().expect("Invalid input");

		println!("Enter a value for altitude(in cm): ");
		let mut input7 = String::new();
		io::stdin().read_line(&mut input7).expect("Failed to read input");
		let altitude:f32 = input7.trim().parse().expect("Invalid input");

		let area_parallelogram:f32 = base*altitude;
		println!("The Area of a Parallelogram with 'base' = {}cm and 'altitude' = {}cm is: {}cm^2", base,altitude,area_parallelogram);
	}
	else if user_input == 3 {     // condition to find the area of cube
		println!("Formula for Area of Cube = 6 * (length of side)^2");
		println!("Enter a value for length of side(in cm): ");
		let mut input8 = String::new();
		io::stdin().read_line(&mut input8).expect("Failed to read input");
		let length_of_side:i32 = input8.trim().parse().expect("Invalid input");

		let area_cube:f32 = 6.0*((length_of_side)^2) as f32;
		println!("The Area of a Parallelogram with 'length of side' =  {}cm is: {}cm^2", length_of_side,area_cube);
	}
	else if user_input == 4 {     // condition to find the volume of cylinder
		println!("Formula for Volume of Cylinder = pi * (radius^2) * height");
		println!("Enter a value for radius(in cm): ");
		let mut input9 = String::new();
		io::stdin().read_line(&mut input9).expect("Failed to read input");
		let radius:i32 = input9.trim().parse().expect("Invalid input");

		println!("Enter a value for height(in cm): ");
		let mut input10 = String::new();
		io::stdin().read_line(&mut input10).expect("Failed to read input");
		let height:f32 = input10.trim().parse().expect("Invalid input");

		let volume_cylinder:f32 = (22.0/7.0)*(radius^2) as f32 *height;
		println!("The Volume of Cylinder with 'radius' = {}cm and 'height' = {}cm is: {}cm^3", radius,height,volume_cylinder);
	}
	else {
		println!("Enter a valid input");
	}
}