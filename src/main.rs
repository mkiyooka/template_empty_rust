use sample::{math, utils};

fn main() {
    let a = 10;
    let b = 5;
    let c = 11; // warning with lsp

    println!("Hello, world1");

    let sum = math::add(a, b);
    println!("{}", utils::format_result("Addition", sum));

    let product = math::multiply(a, b);
    println!("{}", utils::format_result("Multiplication", product));

    let diff = math::subtract(a, b);
    println!("{}", utils::format_result("Subtraction", diff));

    println!("Is {} even? {}", sum, utils::is_even(sum));
}
