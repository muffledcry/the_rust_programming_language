use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Pick a number from 1 to 100:\n");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut pick = String::new();
        io::stdin()
            .read_line(&mut pick)
            .expect("Failed to read line.");
        let guess: u32 = match pick.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}.");

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
