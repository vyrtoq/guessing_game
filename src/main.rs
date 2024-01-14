use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Let's start the game, guess the number!");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
        
        
        // let apples = 5; // immutable
        // let mut bananas = 5; // mutable
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
        
        // shadowing and parsing to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }   
    }
}
