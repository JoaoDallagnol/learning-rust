// 1- Each value in rust has a variable that's called its owner.

fn main() {
    let s1 = String::from("RUST");  
    let len = calculate_length(&s1);
    println!("Lenght of '{}' is {}.", s1, len);
}

// Example: Each value in rust has a variavle that's its owner.
// Example: Each reference is used with the "&" char.
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 2- There can be only one owner at a time.
// This is a error, because s1 is not the owner of the value anymore.
// fn main() {
//     let s1 = String::from("RUST");  
//     let s2 = s1;
//     println!("{}", s1);
// }


// 3- When the owner goes out of scope, the value will be dropped
// fn main() {
//     let s1 = String::from("RUST");  
//     let len = calculate_length(&s1);
//     println!("Lenght of '{}' is {}.", s1, len);
// }

// fn print_lost(s: &string) {
//     println!("{}", s1);
// }