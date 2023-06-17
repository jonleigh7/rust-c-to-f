
use std::io;
fn main() {
    //convert between fahrenheit and celsius 
    println!("Fahrenheit Celsius Converter!");
    
    loop { 
        let mut input = String::new();
        println!("Input 1 if you want to convert F -> C \n 2 if you want to convert C -> F.");
        io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if input == 1 {
            loop { 
                println!("Enter the number in Fahrenheit. \n");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
                let fahrenheit_input: f64 = match input.trim().parse() { 
                    Ok(num) => num,
                    Err(_) => continue,
                };
                print!("Celsius value is: {} \n ", f_to_c(fahrenheit_input));
                break;
            }

        } else if input == 2 {
            loop { 
                println!("Enter the number in Celsius. \n");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Couldn't read input correctly.");
                let celsius_input: f64 = match input.trim().parse() { 
                    Ok(num) => num,
                    Err(_) => continue,
                };
                print!("Fahrenheit value is: {} \n ", c_to_f(celsius_input));
                break;
            }
        } else {
            println!("Specify a number between 1 and 2.");
            continue;
        }
    }
}
fn f_to_c(f:f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
fn c_to_f(c:f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}