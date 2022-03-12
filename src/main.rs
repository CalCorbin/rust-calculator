use std::env::{args, Args};

/// This function takes the input of the user and returns the calculation
fn main() {
  let mut args: Args = args();

  // Grab the numbers and mathematical operators given by the user.
  let first: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();

  // Convert the users input to integers.
  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operate(operator, first_number, second_number);

  // Print the result for the user
  println!("{}", output(first_number, operator, second_number, result))
}

/// This function prints the result of the calculation input by the user.
fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    // Use the format function to make the user input
    // and result readable for the end user.
    format!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    )
}

/// This function takes the operator provided by the user,
/// then matches that operator to the method provided by Rust.
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,

        // _ is the 'default' case here, it captures invalid operators provided by the user.
        _ => panic!("Invalid operator used."),
    }
}
