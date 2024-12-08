use std::io;


fn main() {
    println!("Welcome to Ernst & Young(EY) Global Limited");
    println!("Ernst & Young (EY) Global Limited, is a multinational professional services network with headquarters in London and branch offices all around the world.
EY is one of the largest professional services networks in the world. 
     \nWe are looking to hire the developer with the highest programming experience among the first ten applicants.");

    highest_exp_yrs();
}

fn applicant_name(_name: String) -> String {
    println!("Your Name: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name:String = input1.trim().parse().expect("Invalid input");

    return name;
    
}

fn applicant_exp(_experience: String) -> i32 {
	println!("How many years of programming experience do you have?: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experience:i32 = input2.trim().parse().expect("Invalid input");

    return experience;

}

fn applicant_info() -> (String,i32) {
	let applicant = (applicant_name(String::new()),applicant_exp(String::new()));

	println!("Your Name: {}", applicant.0);
	println!("Your programming experience in years: {}", applicant.1);

	return applicant;

}

fn highest_exp_yrs() {
	let mut applicant_vec : Vec<(String,i32)> = Vec::new();
	let mut x = 0;

	while x < 10 {
		println!("Applicant number {}", x + 1);
		applicant_vec.push(applicant_info());
		x += 1;
	}

	let mut max_exp_yrs = 0;

	for (_,exp_yrs) in &applicant_vec {
		if *exp_yrs > max_exp_yrs {
			max_exp_yrs = *exp_yrs
		}
	}

	let max_vec : Vec<&(String,i32)> = applicant_vec.iter().filter(|(_,max_exp)| *max_exp == max_exp_yrs).collect();

	if max_vec.len() > 1 {
		println!("The developers with the most experience are: ");
		println!("{:?}", max_vec);
		println!("\n{:?} are hired as the new developers for Ernst & Young(EY) Global Limited", max_vec);
	}
	else {
	println!("The developer with the most experience is: ");
	println!("{:?}", max_vec);
	println!("\n{:?} is hired as the new developer for Ernst & Young(EY) Global Limited", max_vec);
	}
}


