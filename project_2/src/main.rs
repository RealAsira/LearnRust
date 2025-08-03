/*
We’ll implement a classic beginner programming problem: a guessing game.
Here’s how it works: the program will generate a random integer between 1 and 100.
It will then prompt the player to enter a guess.
After a guess is entered, the program will indicate whether the guess is too low or too high.
If the guess is correct, the game will print a congratulatory message and exit.
*/


// use io package from standard library
use std::io;
use rand::Rng;

// program entry
fn main() {
    // vars are immutable by default (these are more or less constants ... THERE ARE DIFFERENCES)
    let start_msg = "Guess a number,";
    let guess_range = "1-100";
    println!("{} {}!", start_msg, guess_range);     // pass msg and range indirectly to print (with newline)

    // to use rand, it has to be added to Cargo.lock as a dependency (not built in)
    // it, and dependencies, are then added to "Cargo.lock" after the next `cargo build`
    // to update dependencies, use `cargo update`
    // to update beyond the same 0.X.0 version, manually change in Cargo.toml
        // constant for "target" number to guess
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // mutable variable 
        // instantiated as a string object, running method "new"
        // this creates a new object (guess) of type string which is empty
        let mut guess = String::new();

        // use io package to call "stdin" function
        // .read_line gets a STRING input from user and passes it into "&mut guess"
        // "&mut" modifies the following call (guess) by calling it .asReference and mutable
        // once .read_line is done, the .asRef is completed
        // "&mut guess" = & (reference to data) mut (makes reference mutable) guess (data/var name)
        // .expect ... error handling ... think of "expected x to happen"
        io::stdin()
            .read_line(&mut guess)
            .expect("Should have read user-input for 'guess'");

        // print!("Your guess: {guess}");  // pass guess directly to print (without newline)

        // cast guess as unsigned integer
        let guess:u32 = guess.trim().parse().expect("Please type a number!");

         if secret_number > guess {println!("The secret number is greater than {guess}!");}
         else if secret_number < guess {println!("The secret number is less than {guess}!");}
         else {break;}   // correct guess, exit loop
    }
    println!("Yes! The secret number was {secret_number}.");
}
