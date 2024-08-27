use std::io::{self, Write};

pub fn main() {
    println!(" =============================================");
    println!("||                 Calculator                ||");
    println!(" =============================================");

    let mut input_user = String::new();
    let run: bool = true;
    while run {
        print!("Enter your input (or type 'exit' to quit): ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed before reading input
        input_user.clear();
        io::stdin()
            .read_line(&mut input_user)
            .expect("Failed to read line");
        let input_user = input_user.trim(); // Remove trailing newline characters

        if input_user == "exit" {
            break;
        }

        match parse_and_compute(input_user) {
            Some(result) => println!("Result: {}", result),
            None => println!("Invalid input or unsupported operation."),
        }
    }
}

fn parse_and_compute(input: &str) -> Option<f32> {
    let mut tokens = input.split_whitespace().peekable();
    let mut result = tokens.next()?.parse::<f32>().ok()?;

    while let Some(&operator) = tokens.peek() {
        tokens.next(); // consume the operator
        let next_value = tokens.next()?.parse::<f32>().ok()?;

        result = match operator {
            "+" => add(result, next_value),
            "-" => sub(result, next_value),
            "*" => mul(result, next_value),
            "/" => div(result, next_value),
            _ => return None,
        };
    }

    Some(result)
}

fn add(a: f32, b: f32) -> f32 {
    a + b
}

fn sub(a: f32, b: f32) -> f32 {
    a - b
}

fn mul(a: f32, b: f32) -> f32 {
    a * b
}

fn div(a: f32, b: f32) -> f32 {
    a / b
}
