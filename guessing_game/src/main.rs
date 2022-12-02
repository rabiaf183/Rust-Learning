use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);// rand::thread_rng() brings random number for the seed provided byt the cpu in that exact thread. genrange is for range( start..=last)

    println!("The secret number is: {secret_number}");
    loop{
    println!("Please input your guess.");

    let mut guess = String::new();// new is an associated function of string type which creates a new empty string

    io::stdin()
        .read_line(&mut guess)// read line to read from input read_line is to take whatever the user types into standard input and append that into a string. read line returns either ok or err
        .expect("Failed to read line");

    println!("You guessed: {guess}");
    let guess: u32 = guess.trim().parse().expect("please type a number");// parse converts string type to another defined 
    match guess.cmp(&secret_number){// cmp comapres guess to secret. match 
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too bIG"),
        Ordering::Equal => { println!("You win");
        break;
        {
        }
        }
 
}

