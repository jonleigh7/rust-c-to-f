use std::io;

fn main() {
    // Convert between Fahrenheit and Celsius
    println!("Fahrenheit Celsius Converter!");

    loop {
        // Prompt the user to choose the conversion type
        let mut input = String::new();
        println!("Input 1 if you want to convert F -> C\n2 if you want to convert C -> F.");
        io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1 {
            // Fahrenheit to Celsius conversion
            loop {
                // Prompt the user to enter the temperature in Fahrenheit
                println!("Enter the number in Fahrenheit.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
                let fahrenheit_input: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                // Convert Fahrenheit to Celsius and print the result
                print!("Celsius value is: {}\n", f_to_c(fahrenheit_input));
                break;
            }
        } else if input == 2 {
            // Celsius to Fahrenheit conversion
            loop {
                // Prompt the user to enter the temperature in Celsius
                println!("Enter the number in Celsius.");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
                let celsius_input: f64 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                // Convert Celsius to Fahrenheit and print the result
                print!("Fahrenheit value is: {}\n", c_to_f(celsius_input));
                break;
            }
        } else {
            // Invalid input
            println!("Specify a number between 1 and 2.");
            continue;
        }
    }
}

// Function to convert Fahrenheit to Celsius
fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
