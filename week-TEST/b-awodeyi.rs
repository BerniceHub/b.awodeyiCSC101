use std::io;

fn main() {
	println!("Enter your name: ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let name:i32 = input1.trim().parse().expect("Failed to input");

	println!("Enter your Date of Birth(dd/mm/yyyy): ");

	println!("Date: ");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let d:i32 = input2.trim().parse().expect("Failed to input");

	println!("Month: ");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let m:i32 = input3.trim().parse().expect("Failed to input");

	println!("Year: ");
	let mut input4 = String::new();
	io::stdin().read_line(&mut input4).expect("Failed to read input");
	let y:i32 = input4.trim().parse().expect("Failed to input");

	println!("Enter your email address(e.g mary.longman@gmail.com): ");
	let mut input5 = String::new();
	io::stdin().read_line(&mut input5).expect("Failed to read input");
	let email:i32 = input5.trim().parse().expect("Failed to input");

	println!("Enter your phone number: ");
	let mut input6 = String::new();
	io::stdin().read_line(&mut input6).expect("Failed to read input");
	let phone:i32 = input6.trim().parse().expect("Failed to input");

	println!("Enter your number of siblings: ");
	let mut input7 = String::new();
	io::stdin().read_line(&mut input7).expect("Failed to read input");
	let siblings:i32 = input7.trim().parse().expect("Failed to input");

	println!("Enter your number of children: ");
	let mut input8 = String::new();
	io::stdin().read_line(&mut input8).expect("Failed to read input");
	let children:i32 = input8.trim().parse().expect("Failed to input");

	println!("Enter your medical diagnosis: ");
	let mut input9 = String::new();
	io::stdin().read_line(&mut input9).expect("Failed to read input");
	let diagnosis:&str = input9.trim().parse().expect("Failed to input");

	println!("Enter your village of residence: ");
	let mut input10 = String::new();
	io::stdin().read_line(&mut input10).expect("Failed to read input");
	let residence:&str = input10.trim().parse().expect("Failed to input");

	let a = "alzheimer";
	let b = "arrythmia";
	let c = "chronic kidney disease";
	let di = "diabetes";
	let e = "arthritis";


	let u = "akpabom";
	let v = "ngbauji";
	let w = "atabrikang";
	let x = "okorobilom";
	let yi ="emeremen";


	let d1:i32 = 20;
	let d2:i32 = 5;
	let d3:i32 = 15;
	let d4:i32 = 10;
	let alzheimer:i32 = 1200000;
	let arrythmia:i32 = 550000;
	let ckd:i32 = 1500000;
	let diabetes:i32 = 800000;
	let arthritis:i32 = 450000;

	if *diagnosis == *a && y < 1974 && children > 4 && *residence == *u {
		println!("Name: {}", name); 
		println!("\nDate of Birth: {} / {} / {}", d,m,y); 
		println!("\nEmail Address: {}", email); 
		println!("\nPhone number: {}", phone); 
		println!("\nNumber of siblings: {}", siblings); 
		println!("\nNumber of children: {}", children); 
		println!("\nMedical diagnosis: {}", diagnosis); 
		println!("\nVillage of Residence: {}", residence); 

		let amount1 = (d1/100) * alzheimer;
		println!("The Amount is: N{}", amount1);
	}
	else if *diagnosis == *b && y == 1994 && siblings > 4 && *residence == *v {
		println!("Name: {}", name); 
		println!("\nDate of Birth: {} / {} / {}", d,m,y); 
		println!("\nEmail Address: {}", email); 
		println!("\nPhone number: {}", phone); 
		println!("\nNumber of siblings: {}", siblings); 
		println!("\nNumber of children: {}", children); 
		println!("\nMedical diagnosis: {}", diagnosis); 
		println!("\nVillage of Residence: {}", residence); 

		let amount2 = (d2/100) * arrythmia;
		println!("The Amount is: N{}", amount2);
	}
	else if *diagnosis == *c && y < 1984 && siblings > 3 && children > 3 && *residence == *w {
		println!("Name: {}", name); 
		println!("\nDate of Birth: {} / {} / {}", d,m,y); 
		println!("\nEmail Address: {}", email); 
		println!("\nPhone number: {}", phone); 
		println!("\nNumber of siblings: {}", siblings); 
		println!("\nNumber of children: {}", children); 
		println!("\nMedical diagnosis: {}", diagnosis); 
		println!("\nVillage of Residence: {}", residence); 

		let amount3 = (d3/100) * ckd;
		println!("The Amount is: N{}", amount3);
	}
	else if *diagnosis == *di && y < 1996 && y > 1979 && children == 2 && children == 3 && children == 4 && *residence == *x {
		println!("Name: {}", name); 
		println!("\nDate of Birth: {} / {} / {}", d,m,y); 
		println!("\nEmail Address: {}", email); 
		println!("\nPhone number: {}", phone); 
		println!("\nNumber of siblings: {}", siblings); 
		println!("\nNumber of children: {}", children); 
		println!("\nMedical diagnosis: {}", diagnosis); 
		println!("\nVillage of Residence: {}", residence); 

		let amount4 = (d4/100) * diabetes;
		println!("The Amount is: N{}", amount4);
	}
	else if *diagnosis == *e && y < 1966 && siblings > 5 && children > 5 && *residence == *yi {
		println!("Name: {}", name); 
		println!("\nDate of Birth: {} / {} / {}", d,m,y); 
		println!("\nEmail Address: {}", email); 
		println!("\nPhone number: {}", phone); 
		println!("\nNumber of siblings: {}", siblings); 
		println!("\nNumber of children: {}", children); 
		println!("\nMedical diagnosis: {}", diagnosis); 
		println!("\nVillage of Residence: {}", residence); 

		let amount5 = (d4/100) * arthritis;
		println!("The Amount is: N{}", amount5);
	}
}