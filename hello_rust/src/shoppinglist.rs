use std::io;

pub fn run() {
    println!("How many items will be on your list?");
    let num_items = read_number();
    
    let mut shopping_list = Vec::new();
    
    for _ in 0..num_items {
        println!("What would you like to add to your list?");
        let item = read_string();
        shopping_list.push(item);
    }
    
    println!("Here is your list!");
    for item in &shopping_list {
        println!("{}", item);
    }
    
    println!("Happy shopping!");
}

fn read_number() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}