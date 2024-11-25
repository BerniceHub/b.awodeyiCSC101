use std::io;

fn main() {
    println!("Welcome to Bernie's Eatery!");

    let p = "1 = Poundo Yam/Edikaiko Soup".to_string();
    let f = "2 = Fried Rice & Chicken".to_string();
    let a = "3 = Amala & Ewedu Soup".to_string();
    let e = "4 = Eba & Egusi Soup".to_string();
    let w = "5 = White Rice & Stew".to_string();
    let menu = format!("{} \n{} \n{} \n{} \n{}",p,f,a,e,w);
    println!("Here's the menu for the food items available: ");
    println!("{}", menu);

	println!("Would you like to place an order(for 'yes', enter '1' and for 'no', enter '2'?: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let start:i32 = input.trim().parse().expect("Failed to input");

    if start == 1 {

    	let mut total_cost = 0;

    	loop {      //loop for incase the customer wants to order more than one food

    		println!("What type of food are you ordering(Enter 1,2,3,4 or 5)?: ");
 		    let mut input1 = String::new();
  		    io::stdin().read_line(&mut input1).expect("Failed to read input");
            let food_order:i32 = input1.trim().parse().expect("Failed to input");

            println!("What quantity of food do you want(Enter 1,2,3..)?: ");
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).expect("Failed to read input");
            let quantity:i32 = input2.trim().parse().expect("Failed to input");

            let price_p = 3200;
    		let price_f = 3000;
    		let price_a = 2500;
    		let price_e = 2000;
    		let price_w = 2500;

            let mut price = 0;

    		if food_order == 1 {
    			price = price_p*quantity;
    		}
    		else if food_order == 2 {
    			price = price_f*quantity;
    		}
    		else if food_order == 3 {
    			price = price_a*quantity;
    		}
    		else if food_order == 4 {
    			price = price_e*quantity;
    		}
    		else if food_order == 5 {
    			price = price_w*quantity; 
    		}
    		else {
    			println!("Enter either 1,2,3,4 or 5")
    		}

            if price > 0 {
                println!("The price of your order is: N{}", price);
                total_cost += price;
            }

    		println!("Would you like to order anything else(for 'yes', enter '1' and for 'no', enter '2')?: ");
            let mut input3 = String::new();
            io::stdin().read_line(&mut input3).expect("Failed to read input");
            let more_food:i32 = input3.trim().parse().expect("Failed to input");
    		
            if more_food==2 {
           	    break;
            }           
    	}

        if total_cost > 10_000 {
            let discount = 0.05 * total_cost as f64;
            let final_price = total_cost as f64 - discount;
            println!("Total cost: N{}", total_cost);
            println!("The discount price is: N{}", discount);
            println!("The final price for your order(after discount) is: N{}", final_price);
        }
        else{
            println!("The total cost for your order is: N{}", total_cost);
        }

        println!("Thanks for your patronage!");
        println!("Have a nice day!!");
    }
}