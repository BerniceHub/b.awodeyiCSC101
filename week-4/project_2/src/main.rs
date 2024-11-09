//Rust program to determine the annual incentive of employees

use std::io;

fn main() {
    println!("Enter the number of years of employees’ experience:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let experience = input.trim().parse().expect("Failed to input");

    println!("Enter the age of the employee: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age = input1.trim().parse().expect("Failed to input");

    if experience >= 5 && age >= 40 {
    	let incentive1 = N1,560,000;
    	println!("This employee is qualified for an annual incentive of: {}", incentive1);
    }
    else if experience >= 5 && age >= 30 && age < 40 {
    	let incentive2 = N1,480,000;
    	println!("This employee is qualified for an annual incentive of: {}", incentive2);
    }
    else if experience >= 5 && age < 28 {
    	let incentive3 = N1,300,000;
    	println!("This employee is qualified for a monthly incentive of: {}", incentive3);
    }
    else {      //if employee is not experienced 
    	let incentive4 = N100,000;
    	println!("This employee is qualified for an annual incentive of: {}", incentive4);
    }
}
