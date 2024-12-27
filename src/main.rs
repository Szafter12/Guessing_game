use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");
    let mut health: u32 = 10;
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); 

    loop {
        println!("Please input your guess.");

        let mut guess:String = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
    
        println!("Your guess {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                health -= 1;
                println!("Health: {}", health);
                if health <= 0 {
                    println!("You lose!");
                    break;
                }
            },
            Ordering::Greater => {
                println!("Too big!");
                health -= 1;
                println!("Health: {}", health);
                if health <= 0 {
                    println!("You lose!");
                    break;
                }
            },
            Ordering::Equal => {
                println!("You win with {} HP", health);
                break;
            }
        }
    }

}