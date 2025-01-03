use std::io;

fn main() {
    println!("Convert temperatures between Fahrenheit and Celsius.");

    loop {
        println!("Enter a '?', or the temperature.");

        let mut fahrenheit = String::new();
        let mut celsius = String::new();

        println!("Fahrenheit: ");

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line.");

        println!("Celsius: ");

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line.");

        if fahrenheit != "?" && celsius != "?" {
            println!("One of two values needs to be unknown.");

            continue;
        }

        let fahrenheit: u64 = fahrenheit
            .trim()
            .parse()
            .expect("Fahrenheit is not a number.");
        let celsius: u64 = celsius.trim().parse().expect("Fahrenheit is not a number.");
    }
}
