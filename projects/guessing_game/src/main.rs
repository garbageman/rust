use std::io;

/**
* This is the main function of the program, it will execute when the compiled code is run.
*/
fn main() {
    /* Standard println function */
    println!("Guess the number!");

    println!("Please input your guess");

    /* Creating a mutable object for storing the input */
    let mut guess = String::new();

    /* Using std in to read the next line of input */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    /* Finally output the line with the guess */
    println!("You guessed {}", guess);
}
