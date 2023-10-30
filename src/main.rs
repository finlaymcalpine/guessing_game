// bringing io library into scope of file from standard library
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // thread_rng is telling the program to use an RNG generated from an OS seed. So its returning an RNG?
    // gen_range() method then uses the RNG to generate a value from the given range, inclusive of start/end
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // No longer want to print this line. We have to get the number right to break the loop, though!
    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess:");

        // create guess variable to store input: values are immutable by deafult so we need to include mut.
        // we are binding that variable to a new, empty instance of a string, called using String::new().
        // taking the type String, and calling the associated function ::new() to create the string.
        let mut guess = String::new();

        // calling stdin function from the io library. could also not import io at top and use std::io::stdin.
        // read_line function takes input from stdin and appends it to string given in the arguments
        // hence string variable defined above needs to be mutable.
        // & says we are passing a reference to guess, not the actual guess variable. &mut says this ref is mutable.
        // expect is an error handling method on the Result value returned by read_line(). If the Result instance is
        // Err, then it will pass the error message we provide in the argument. If Result instance is Ok, then expect()
        // returns the value that Ok is holding.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // added line to assign guess input string to a new guess integer for use in comparison below.
        // I think the guess variable here is different to, but shadows, the string above.
        // We are now matching the parsed input Enum return to the relevant action. If Enum is Ok then use number.
        // If Enum is Err (of any type using _), then we go on to the next loop iteration and ask again for a number.
        // We're creating a way of error handling here that doesn't involve just crashing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //.expect("Please type a number!"); PREVIOULSY USED for handling parsing failures

        // {} is a formatting like an F-string in python, so here guess variable is printed in {}.
        println!("You guessed: {guess}");

        // applies comparison method cmp() to the guess, and uses Ordering enum to see what is generated.
        // match uses pattern match to look for the matching pattern from the output of cmp().
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
