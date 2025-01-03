use std::io;

fn main() {
    const FAHRENHEIT_MULTIPLIER: f64 = 9.0 / 5.0;
    const FAHRENHEIT_OFFSET: f64 = 32.0;

    println!("Convert temperatures between Fahrenheit and Celsius.");

    let option = choose_option();

    match option {
        1 => println!("Celsius:"),
        2 => println!("Fahrenheit:"),
        _ => unreachable!(),
    };

    let temp = get_temp();

    match option {
        1 => {
            let fahrenheit = (temp * (FAHRENHEIT_MULTIPLIER) + FAHRENHEIT_OFFSET).round();

            println!("{temp} Celsius is {fahrenheit} Fahrenheit.");
        }
        2 => {
            let celsius = ((temp - FAHRENHEIT_OFFSET) / FAHRENHEIT_MULTIPLIER).round();

            println!("{temp} Fahrenheit is {celsius} Celsius.");
        }
        _ => unreachable!(),
    };
}

// Get temp from user input.
fn get_temp() -> f64 {
    loop {
        let mut temp_buf = String::new();
        io::stdin()
            .read_line(&mut temp_buf)
            .expect("Failed to read line.");

        let temp: f64 = match temp_buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature!");
                continue;
            }
        };

        return temp;
    }
}

// Choose option, gets used input and returns option value.
fn choose_option() -> u64 {
    loop {
        let mut option_buf = String::new();

        println!("Choose an option:");
        println!("  1. Calculate Fahrenheit");
        println!("  2. Calculate Celsius");
        println!("Enter choice [1/2]:");

        io::stdin()
            .read_line(&mut option_buf)
            .expect("Failed to read line.");

        let option: u64 = match option_buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid option!");
                continue;
            }
        };

        if option == 1 || option == 2 {
            return option;
        }
    }
}
