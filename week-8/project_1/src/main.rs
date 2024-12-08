use std::io;

fn main() {
    let branch_vec = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    let aps1 = vec!["Intern","Nil","Paralegal","Placement"];
    let aps2 = vec!["Administrator","Research Assistant","Junior Associate","Classroom Teacher"];
    let aps3= vec!["Senior Administrator","PhD Candidate","Associate","Snr Teacher"];
    let el4 = vec!["Office Manager","Post_Doc Researcher","Senior Associate 1-2","Leading Teacher"];
    let el5 = vec!["Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"];
    let ses = vec!["CEO","Dean","Partner","Principal"];
    let _positions = vec!["APS 1-2","APS 3-5","APS 5-8","EL1 8-10","EL2 10-13","SES"];


    println!("Welcome to the Public Service APS level checker");
    println!("Below are the branches of the Public Service: ");
    let mut count = 0;
    for index in 0..4 {
    println!("{} {}", count, branch_vec[index]);
    count += 1;
    }
    println!("What branch are you in? (Enter index 0,1,2 or 3): ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let branch:usize = input1.trim().parse().expect("Invalid input");

    let mut num = 0;

    println!("The following are the various levels for the branch '{}':", branch_vec[branch]); 
    
    
    println!("{} {}", num, aps1[branch]);
    num += 1;
    println!("{} {}", num, aps2[branch]);
    num += 1;
    println!("{} {}", num, aps3[branch]);
    num += 1;
    println!("{} {}", num, el4[branch]);
    num += 1;
    println!("{} {}", num, el5[branch]);
    num += 1;
    println!("{} {}", num, ses[branch]);


    println!("What level are you at? (Enter index 0,1,2,3,4 or 5):");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let level:i32 = input2.trim().parse().expect("Invalid input");

    if branch == 1 && level == 0 {
    	print!("Your Staff level cannot be validated");
    }

    else {
    	println!("How many years of experience do you have? :");
    	let mut input3 = String::new();
    	io::stdin().read_line(&mut input3).expect("Failed to read input");
    	let experience:i32 = input3.trim().parse().expect("Invalid input");

    	if level == 0 {
    		let lev = aps1[branch];
    		let post = "APS 1-2";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else if level == 1 {
    		let lev = aps2[branch];
    		let post = "APS 3-5";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else if level == 2 {
    		let lev = aps3[branch];
    		let post = "APS 5-8";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else if level == 3 {
    		let lev = el4[branch];
    		let post = "EL1 8-10";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else if level == 4 {
    		let lev = el5[branch];
    		let post = "EL2 10-13";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else if level == 5 {
    		let lev = ses[branch];
    		let post = "SES";
    		println!("Branch: {} \nLevel: {} \nExperience: {} years \nYour Position is: {}", branch_vec[branch], lev, experience, post);
    	}
    	else {
    		println!("Enter a valid input");
    	}
	}

}