fn main() {
	let p = 1000;
	let r = 1;
	let t = 2;

	//simple interest
	let a = p * ( 1 + (r / 100)) ^ t;
	println!("Amount is {}", a);
	let si = a - p;
	println!("Simple interest is {}", si);

}