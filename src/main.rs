use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input the number");
    let rand_n: i32 = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    loop {
        guess = String::from("");
        io::stdin().read_line(&mut guess).expect("Failed to read number");
        println!("You guess is : {}", guess.trim());
        let secret_number: i32 = match guess.trim().parse() {
            Ok(t) => { t }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        match rand_n.cmp(&secret_number) {
            Ordering::Less => println!("Your number is greater then the number."),
            Ordering::Greater => println!("Your number is less then the number."),
            Ordering::Equal => {
                println!("Your number is Equal to the number.");
                break;
            }
        }
    }
}
