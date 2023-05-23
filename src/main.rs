use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("This is a guessing game");
    println!("Please enter your guess");
    let secret = rand::thread_rng().gen_range(1..=100);
    loop{
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Error in reading the input");
    println!("Your guess is : {guess}");
    let guess : u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    match guess.cmp(&secret) {
        Ordering::Less => println!("Guess is less"),
        Ordering::Greater => println!("Guess is greater"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    }


}
