fn main() {
    // A simple loop that could be written differently
    for _ in 0..100 {
        // Placeholder code
        println!("Hello, world!");
    }

    // An example of code that might trigger a pedantic lint:
    let x = 42;
    println!("The answer is {}", x);
}

fn redundant_return_type() -> i32 {
    let result = 10;
    return result;
}
