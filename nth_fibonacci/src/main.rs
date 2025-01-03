use std::io;

fn main() {
    let num = get_num();

    let result = fibonacci(num);

    println!("Number: {result}");
}

fn fibonacci(n: i32) -> u64 {
    if n < 0 {
        panic!("Negative number!");
    }

    match n {
        0 => 0,
        1 | 2 => 1,
        3 => 2,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
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
