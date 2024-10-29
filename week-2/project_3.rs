fn main() {
	let p = 510000; //original price
	let r = 5; // rate of depreciation
	let n = 3; //time

    //value of TV after 3 years 
	let a = p * ( 1 - (r / 100)) ^ n; //formula for amount
	println!("The value of the TV after 3 years is {}", a);
}