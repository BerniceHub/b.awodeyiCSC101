//Rust program to determine the annual incentive of employees

use std::io;

fn main() {
    println!("Enter the number of years of employees’ experience:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let experience:i32 = input.trim().parse().expect("Failed to input");

    println!("Enter the age of the employee: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:i32 = input1.trim().parse().expect("Failed to input");

    if experience >= 2 && age >= 40 {
    	let incentive1:i32 = 1_560_000;
    	println!("This employee is qualified for an annual incentive of(in Naira): {}", incentive1);
    }
    else if experience >= 2 && age >= 30 && age < 40 {
    	let incentive2:i32 = 1_480_000;
    	println!("This employee is qualified for an annual incentive of(in Naira): {}", incentive2);
    }
    else if experience >= 2 && age < 28 {
    	let incentive3:i32 = 1_300_000;
    	println!("This employee is qualified for a monthly incentive of(in Naira): {}", incentive3);
    }
    else {      //if employee is not experienced 
    	let incentive4:i32 = 100_000;
    	println!("This employee is qualified for an annual incentive of(in Naira): {}", incentive4);
    }
}
