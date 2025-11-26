use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Parse error");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("to large"),
            Ordering::Equal => {
                println!("win");
                break;
            }
        };
    }
}