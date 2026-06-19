use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    let mut guess = String::new();
    let mut guess_number: i32;
    loop {
        println!("please input your guess.");
        guess.clear();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess_number = match guess.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Please input a valid number.");
                continue;
            }
        };
        match guess_number.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Well done! {guess_number} is the random number");
                break;
            }
        }
    }

}