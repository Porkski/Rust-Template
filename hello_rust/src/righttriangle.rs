use std::io;

pub fn run() {
    let mut input = String::new();
    
    println!("What is the height of the right triangle?");
    io::stdin().read_line(&mut input).unwrap();
    let height = input.trim().parse::<i32>().unwrap();
    
    for i in 0..height {
        for j in 0..i + 1 {
            print!("* ");
        }
        println!();
    }
}