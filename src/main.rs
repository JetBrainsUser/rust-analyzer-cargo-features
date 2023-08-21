fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(feature = "advanced")]
fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    let a = 5;
    let b = 3;

    println!("Add: {} + {} = {}", a, b, add(a, b));
    println!("Subtract: {} - {} = {}", a, b, subtract(a, b));

    #[cfg(feature = "advanced")]
    println!("Factorial of {} is {}", a, factorial(a as u32));
}
