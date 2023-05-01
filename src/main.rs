#![warn(clippy::all, clippy::pedantic)]

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn play_guessing_game() {
    fn gen_range(start: u32, end: u32, include_end: bool) -> u32 {
        if include_end {
            thread_rng().gen_range(start..=end)
        } else {
            thread_rng().gen_range(start..end)
        }
    }

    let random_number = gen_range(1, 10, false);

    loop {
        let mut user_guess = String::new();
        io::stdin()
            .read_line(&mut user_guess)
            .expect("could not read input");

        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("err {}", err);
                0
            }
        };

        match user_guess.cmp(&random_number) {
            Ordering::Less => {
                println!("opps too low")
            }
            Ordering::Equal => {
                println!("winnner winner chicken dinner");
                break;
            }
            Ordering::Greater => {
                println!("opps too high")
            }
        }
    }
}

fn loop_though_week() {
    const WEEK: [&str; 7] = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];

    for day in WEEK.iter() {
        println!("day {}", day);
    }
}
fn main() {
    println!("guess a number from 1 to 10");

    play_guessing_game();
}
