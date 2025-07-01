use std::io;

pub fn run() {
    let mut input = String::new();
    
    println!("Enter the first operand:");
    io::stdin().read_line(&mut input).unwrap();
    let x: i32 = input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter the second operand:");
    io::stdin().read_line(&mut input).unwrap();
    let y: i32 = input.trim().parse().unwrap();
    
    input.clear();
    println!("Enter the operation:");
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim();
    
    if op == "add" || op == "+" {
        println!("{}", x + y);
    }
    if op == "sub" || op == "-" {
        println!("{}", x - y);
    }
    if op == "mul" || op == "*" {
        println!("{}", x * y);
    }
    if op == "div" || op == "/" {
        println!("{}", x / y);
    }
}