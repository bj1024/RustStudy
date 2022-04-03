use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let apples = 5; // immutable
    let mut bananas = 14; // mutable
    println!("apples={:010} bananas={:#X}.",apples,bananas);
    bananas += 1;
    println!("apples={} bananas={}.",apples,bananas);
    

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}