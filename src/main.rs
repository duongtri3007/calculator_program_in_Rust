use std::io;

// Calculation functions
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

// Helper function to read a single integer input from the user
fn read_input() -> i32 {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read input");
    let result: i32 = buffer.trim().parse().expect("Invalid number entered");
    result
}

// Main function
fn main() {
    loop {
        // Print the menu
        println!("\n--- Calculator ---");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("-----------------");
        
        println!("Choose an option (1-4):");
        let choice = read_input();

        // Get the two numbers
        println!("Enter the first number:");
        let num1 = read_input();

        println!("Enter the second number:");
        let num2 = read_input();

        // Perform the calculation based on user choice
        if choice == 1 {
            println!("Result: {}", add(num1, num2));
        } else if choice == 2 {
            println!("Result: {}", subtract(num1, num2));
        } else if choice == 3 {
            println!("Result: {}", multiply(num1, num2));
        } else if choice == 4 {
            if num2 == 0 {
                println!("Division by zero is not allowed!");
            } else {
                println!("Result: {}", divide(num1, num2));
            }
        } else {
            println!("Error!!! TRY AGAIN!");
        }
    }
}