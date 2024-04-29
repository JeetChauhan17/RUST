use std::io;

fn main() {
    println!("Welcome to the guessing game! ");
    println!("Guess a number between 1 to 10:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
    .expect("Failed to read line.");

    println!("You guessed: {guess}");
}
