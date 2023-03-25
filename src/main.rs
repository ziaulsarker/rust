use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn get_random_num(start: u8, end: u8) -> u8 {
    let mut rng = thread_rng().gen_range(start..=end);
    rng
}
fn main() {
    let random_number = get_random_num(0, 10);
    println!("enter a number from 0 to 10");

    println!("{}", random_number);

    loop {
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_input: u8 = user_input
            .trim()
            .parse()
            .expect("opps cant parse user input");

        match user_input.cmp(&random_number) {
            Ordering::Less => println!("sorry to low"),
            Ordering::Equal => {
                println!("winner winner chicken dinner");
                break;
            }
            Ordering::Greater => println!("sorry to high"),
        }
    }
}
