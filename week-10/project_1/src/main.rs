struct Laptops {
    name:String,
    _no_of_laptops:i32,
    cost:i32
}

fn main() {
    let laptop1 = Laptops{
        name:String::from("HP Laptops"),
        _no_of_laptops:10,
        cost:650000
    };
    let laptop2 = Laptops{
        name:String::from("IBM Laptops"),
        _no_of_laptops:6,
        cost:755000
    };
    let laptop3 = Laptops{
        name:String::from("Toshiba Laptops"),
        _no_of_laptops:10,
        cost:550000
    };
    let laptop4 = Laptops{
        name:String::from("Dell Laptops"),
        _no_of_laptops:4,
        cost:850000
    };

    let total_cost1 = laptop1.cost*3;
    let total_cost2 = laptop2.cost*3;
    let total_cost3 = laptop3.cost*3;
    let total_cost4 = laptop4.cost*3;
    let final_total = total_cost1 + total_cost2 + total_cost3 + total_cost4;
    display_cost(laptop1);
    display_cost(laptop2);
    display_cost(laptop3);
    display_cost(laptop4);
    println!("The total cost is {}",final_total);
}

fn display_cost(laptop: Laptops) {
    let costs = laptop.cost*3;
    println!("The cost of 3 {} is {}",laptop.name,costs);
}

