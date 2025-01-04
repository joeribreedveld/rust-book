use std::io;

fn main() {
    let num = get_num();

    let result = fibonacci(num);

    println!("Number: {result}");
}

// fn fibonacci_recursive(n: i32) -> u64 {
//     if n < 0 {
//         panic!("Negative number!");
//     }

//     match n {
//         0 => 0,
//         1 | 2 => 1,
//         3 => 2,
//         _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
//     }
// }

fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("Negative number!");
    } else if n == 0 {
        panic!("Zero is not possible!");
    } else if n == 1 {
        return 1;
    }

    // Totaal
    let mut current = 0;

    // Parent
    let mut grandparent = 0;
    let mut parent = 1;

    // Begin bij 1 want eerste iteratie staat al klaar in variable initialisatie
    for _i in 1..n {
        // Tel de parent en grandparent bij elkaar op
        current = grandparent + parent;
        grandparent = parent;
        parent = current;
    }

    // Return current van vorige niet huidige
    current
}

fn get_num() -> i32 {
    loop {
        println!("Enter nth fibonacci number:");

        let mut nth_fibo_buf = String::new();

        io::stdin()
            .read_line(&mut nth_fibo_buf)
            .expect("Failed to read line.");

        let nth_fibo: i32 = match nth_fibo_buf.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                continue;
            }
        };

        return nth_fibo;
    }
}
