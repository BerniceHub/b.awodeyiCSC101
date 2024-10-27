fn main() {
	let t = 2 * 450000; //Toshiba
	let m = 1 * 1500000; //Mac
	let h = 3 * 750000; //Hp
	let d = 3 * 2850000; //Dell
	let a = 1 * 250000; //Acer

	let s = t + m + h + d + a; //Sum of Sales Record 
	println!("The sum of the Sales Record is {}", s);
	let av = s/(2+1+3+3+1); //Average 
	println!("The average is {}", av);
}