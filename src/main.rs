#![warn(clippy::all, clippy::pedantic)]

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn play_guessing_game() {
    println!("guess a number from 1 to 10");
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

    for day in WEEK {
        println!("day {}", day);
    }
}

enum Converter {
    FToC,
    CToF,
}

fn convert_temp(temp: f32, converter: Converter) -> f32 {
    let temp = match converter {
        Converter::FToC => (temp - 32.0) * 5.0 / 9.0,
        Converter::CToF => (temp * 9.0 / 5.0) + 32.0,
    };
    temp
}

fn generate_nth_fib_numeber(num: usize) -> usize {
    let result = if num <= 2 {
        1
    } else {
        generate_nth_fib_numeber(num - 1) + generate_nth_fib_numeber(num - 2)
    };
    result
}

fn main() {
    //play_guessing_game();

    // loop_though_week()

    // let t = generate_nth_fib_numeber(6);

    // println!("temo is {}", t);
    let x = hasAZero(51);
    println!("{}", x);

    fizz_buzz(15);
}

fn hasAZero(num: isize) -> bool {
    let number_str = num.to_string();

    for i in 0..number_str.chars().count() {
        if number_str.chars().nth(i).unwrap() == '0' {
            return true;
        }
    }
    return false;
}

fn fizz_buzz(num: isize) {
    for i in 0..=num {
        if (i % 3 == 0 && i % 5 == 0) {
            println!("fizz buzz: {}", i)
        } else if (i % 3 == 0) {
            println!("fizz: {}", i)
        } else if (i % 5 == 0) {
            println!("buzz: {}", i)
        } else {
            println!("{}", i)
        }
    }
}
