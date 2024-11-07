use rand::Rng; // importing rand lib.
use std::cmp::Ordering; // importing cmp lib.
use std::io; // importing io lib.

// Note: cargo doc --open -> this command will build the docs of all the crates used in the poject.

fn main() {
    println!("Guess the number!");

    // generating random number between 1 to 100.
    // syntax: trait::function().method();
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop {} will create an infinite loop
    loop {
        // println!("The secret number is: {secret_number}");
        println!("Please enter the number: ");

        // taking user input....

        // In rust variables are immutable by default, so we need to make it mutable using mut keyword.
        // Use the syntax String::new() to create a new instance of a string.
        let mut guess = String::new();

        // .read_line will append the readed user input to the assigned mutable variable that's why we pass the ref. of the mutable variable.
        // .expect is used for error handling.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        // printing the user input...
        println!("You guessed: {}", guess);

        // cool syntax!!
        // let x = 5;
        // let y = 10;

        // println!("The value of x = {x} and value of y + 2 = {}", y + 2);

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error:{}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
