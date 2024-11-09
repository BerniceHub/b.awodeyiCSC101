//Rust program to find the roots of a quadratic equation

use std::io;

fn main() {
    println!("Enter a value for a: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let a:f32 = input1.trim().parse().expect("Failed to input");


   println!("Enter a value for b: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let b:f32 = input2.trim().parse().expect("Failed to input");

    println!("Enter a value for c: ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let c:f32 = input3.trim().parse().expect("Failed to input");

    //finding the discriminant
    let discriminant:f32 = b * b - 4.0 * a * c;
    println!("The discriminant is {}", discriminant);

    //finding the number of roots using the discriminant
    if discriminant > 0.0 {       //discriminant is positive
    	let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
    	let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
    	println!("There are two distinct roots: {} and {}", root1, root2);
    }
    else if discriminant == 0.0 {         //discriminant is 0.0
    	let root = -b / (2.0 * a);
    	println!("There is exactly one real root: {}", root);
    }
    else {      //discriminant is negative
    	let real_part = -b / (2.0 * a);
        let imaginary_part = (discriminant.abs()).sqrt() / (2.0 * a);
        println!(
            "There are no real roots: {} + {}i and {} - {}i",
            real_part, imaginary_part, real_part, imaginary_part)
    }
}
