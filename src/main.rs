use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("stating a random number generator");
    let random_number = generate_random_number(0, 10);
    println!("guess a random number from 0 to 10");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to guess a random number");

        let guess: i32 = guess
            .trim()
            .parse()
            .expect("failed to guess a random number");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("opp to low bitch"),
            Ordering::Equal => {
                println!("winner winner chicker dinner");
                break;
            }
            Ordering::Greater => println!("too hight bitch"),
        }
    }
}

fn generate_random_number(start: i32, end: i32) -> i32 {
    thread_rng().gen_range(start..=end)
}
