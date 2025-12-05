use std::io::stdin;

use rand::Rng;

fn main() {
    let number: i32 = rand::rng().random_range(1..=15);
    loop {
        println!("Pick a number (1 to 15) >>>");

        let mut line = String::new();
        let input = stdin().read_line(&mut line);
        let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());

        match guess {
            None => println!("enter a number..."),
            Some(n) if n == number => {
                println!("Bravo! You guess it");
                break;
            }
            Some(n) if n < number => println!("Too low"),
            Some(n) if n > number => println!("Too high"),
            Some(_) => println!("Error!"),
        }
    }
}
