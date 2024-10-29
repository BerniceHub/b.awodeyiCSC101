fn main() {
	let p = 520000000; //principal
	let n = 5; //time
	let r = 10; //rate

    //compound interest
	let a = p * ( 1 + (r / 100)) ^ n; //formula for amount
	println!("The Amount is {}", a);
	let ci = a - p; //formula for compound interest
	println!("The Compound Interest is {}", ci);
}