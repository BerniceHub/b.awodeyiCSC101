use std::io;

fn add(a:f32, b:f32) {
	let sum = a + b;

	println!("Sum of A and B is: {}", sum);
}

fn main() {

    let mut input1 = String::new();
    println!("Enter input for parameter A: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter input for parameter B: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Invalid input");

    // call add function with arguments 
    add(a, b);
}
