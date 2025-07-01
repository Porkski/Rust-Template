use std::io;
use rand::Rng;

pub fn run() {
    let x = rand::thread_rng().gen_range(1..=100);
    let mut guess = -1;
    let mut tries = 0;
    
    while guess != x {
        let mut input = String::new();
        println!("Please enter a number between 1-100.");
        io::stdin().read_line(&mut input).unwrap();
        guess = input.trim().parse().unwrap();
        tries += 1;
        
        if guess > x {
            print!("Too high! ");
        }
        if guess < x {
            print!("Too low! ");
        }
        if tries == 7 && guess != x {
            println!("You're out of tries, you lose.");
            break;
        } else if guess != x {
            println!("Try again.");
        }
    }
    
    if guess == x {
        println!("You won!");
    }
}