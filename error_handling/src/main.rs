// Error handling techniques
// Approach 1
// enum Option<T> { // Define the generic Option type
//     Some(T), // Representes a value
//     None, // Representes no value
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

// Approach 2
// enum Result<T, E> { //Define the generic result type
//     Ok(T), //Represents a value
//     Err(E) //Represents an error
// }

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator /  denominator)
    }
}

fn main() {
    let result = divide(10.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divive by Zero!"),
    }

    match divide_result(100.23, 73.98) {
        Ok(result) => println!("Resust: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
