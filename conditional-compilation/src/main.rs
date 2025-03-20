#[cfg(feature = "sum")]
fn sum(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(feature = "sub")]
fn sub(a: f64, b: f64) -> f64 {
    a - b
}

#[cfg(feature = "div")]
fn div(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

#[cfg(feature = "mul")]
fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn main() {
    #[cfg(feature = "sum")]
    println!("Sum: {}", sum(5.0, 3.0));

    #[cfg(feature = "sub")]
    println!("Subtraction: {}", sub(5.0, 3.0));

    #[cfg(feature = "div")]
    match div(6.0, 2.0) {
        Ok(result) => println!("Division: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    #[cfg(feature = "mul")]
    println!("Multiplication: {}", mul(5.0, 3.0));

    println!("\nEnabled operations:");
    #[cfg(feature = "sum")]
    println!("- Sum is enabled");
    #[cfg(feature = "sub")]
    println!("- Subtraction is enabled");
    #[cfg(feature = "div")]
    println!("- Division is enabled");
    #[cfg(feature = "mul")]
    println!("- Multiplication is enabled");

    #[cfg(not(any(feature = "sum", feature = "sub", feature = "div", feature = "mul")))]
    println!("No operations enabled! Use --features to enable functions.");
}
