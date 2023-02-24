use std::io;
use rand::Rng;

/// Entry-point
fn main() {
    println!("Let's do some quick maths!");

    // Run the app, we do this in a function to allow it to be fired again.
    run_app();
}

/// Queries the user with a question and returns if their answer was valid or not
fn run_question() -> bool {
    // Generate the question and get the answer
    let ans = gen_sum();

    // Now we handle the user input, not the question gen itself
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read input.");

    let trimmed = input_text.trim();
    let mut valid = false;
    match trimmed.parse::<i32>() {
        Ok(i) => {
            valid = i == ans; // Set validity of the answer
        },
        Err(..) => println!("Not a valid integer!"),
    }

    return valid; // Return it
}

/// Runs the application, manages the amount of correct and answered questions
fn run_app() {
    // Set answered and correct values
    let mut answered = 0;
    let mut correct = 0;

    // Iterate over range of 1 to 11 (because that's how numbers work in programming)
    for i in 1..11 {
        // Print question number and run the question
        println!("[Question {}]", i);
        let valid = run_question();

        answered = answered+1; // Increment answered
        if valid { // Check if they answered correctly
            correct = correct+1; // Increment correct
        }
    }

    println!("You've answered {} questions and got {} correct.", answered, correct);
}

/// Enum of valid operations to use, I've decided to not do division otherwise
/// I'd have to make a check whether the numbers are equally divisible.
#[derive(Debug, Clone, Copy)]
enum Operation {
    Add = 0,
    Subtract = 1,
    Multiply = 2
}

/// Generates a question, prints it and returns the valid answer
fn gen_sum() -> i32 {
    // Make new thread-safe RNG
    let mut rng = rand::thread_rng();

    // Gen a operation, get a number between 0,3 (the enum range)
    let rand_op_num = rng.gen_range(0..2);

    // Match convert the num into a operation
    let op = match rand_op_num { // Opt to use a safe conversion
        0 => Operation::Add,
        1 => Operation::Subtract,
        2 => Operation::Multiply,
        _ => {
            panic!("WTF!?!?!?!");
        }
    };

    // Gen two numbers between 1,100 and 1,50
    let num_one = rng.gen_range(1..100);
    let num_two = rng.gen_range(1..50);

    // Now calculate the answer, match the operator and perform the operation
    // We also print out the question in this function
    let ans: i32;
    match op {
        Operation::Add => {
            println!("What is {} + {}?", num_one, num_two);
            ans = num_one + num_two;
        },
        Operation::Subtract => {
            println!("What is {} - {}?", num_one, num_two);
            ans = num_one - num_two;
        },
        Operation::Multiply => {
            println!("What is {} x {}?", num_one, num_two);
            ans = num_one * num_two;
        },
    }

    return ans;
}
