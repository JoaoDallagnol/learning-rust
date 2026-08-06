// Variables and Mutability
fn main() {
    let _a: u16 = 5;
    //_a = 15; | cannot assign twice to immutable variable

    let mut _b = 10;
    _b = 15;
    println!("Value of a is {}", _a);
    println!("Value of b is {}", _b);
}
