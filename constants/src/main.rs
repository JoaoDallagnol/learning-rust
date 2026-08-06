// Constants
fn main() {
    let mut x = 5;

    // Declaration is always CAPS and with the type annotation
    const Y: i32 = 10;

    println!("The value of x is: {}", x);
    println!("The value of Y is: {}", Y);
    println!("The value of PI is: {}", PI);
    println!("The value of 3 Hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// You can declare a constants with a type annotation outside the scope
const PI: f64 = 3.141592653;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

