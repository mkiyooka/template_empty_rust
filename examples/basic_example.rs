use sample::{
    math::{self, CalcError},
    utils,
};

fn main() {
    println!("=== Basic Example of sample library ===\n");

    // Using add function
    let result = math::add(5, 3);
    println!("{}", utils::format_result("5 + 3", result));

    // Using math module
    let result2 = math::multiply(10, 20);
    println!("{}", utils::format_result("10 * 20", result2));

    let result3 = math::subtract(100, 25);
    println!("{}", utils::format_result("100 - 25", result3));

    match math::divide(10, 2) {
        Ok(v) => println!("{}", utils::format_result("10 / 2", v)),
        Err(_) => println!("10 / 0: Error"),
    }

    match math::divide(10, 0) {
        Ok(v) => println!("{}", utils::format_result("10 / 0", v)),
        Err(_) => println!("10 / 0: Error"),
    }

    // Using utils module
    println!("\nChecking even/odd numbers:");
    for i in 1..=5 {
        println!(
            "{} is {}",
            i,
            if utils::is_even(i) { "even" } else { "odd" }
        );
    }
}
