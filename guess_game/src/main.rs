use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts: i32 = 0;

    loop {
        let mut guess = String::new();
        println!("Type your guess:");
        io::stdin().read_line(&mut guess).expect("read_line failed");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempts = attempts + 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win with {attempts} attempts!");
                break;
            }
        }
    }
}
