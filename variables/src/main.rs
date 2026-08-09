// Variables and Mutability
fn main() {
    let _a: u16 = 5;
    //_a = 15; | cannot assign twice to immutable variable

    let mut _b = 10;
    _b = 15;
    println!("Value of a is {}", _a);
    println!("Value of b is {}", _b);

    // ---- Primitive data types ----
    // int, float, bool, char
    let name: &str = "João";

    // Integer
    // Rust has signed (+ and -) and unsigned integer (only+)
    // types of different sizes
    // i8, i16, i32, i64, i128
    // u8, u16, u32, u64, u128 

    // let a = 98_222; // Decimal
    // let b = 0xff; // Hex
    // let c = 0o77; // Octal
    // let d = 0b1111_0000; // Binary
    // let e = b'A'; // Byte (u8 only)

    // Floats [Floating Point Types]
    // f32, f64

    // ---- Compound data types ----
    //arrays, tuples, slices, and strings (slice string)
    let numbers: [i32; 5] = [1,2,3,4,5];
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];

    // Tuples
    let human: (&str, i32, bool) = ("Alice", 30, false);
    
    //Slices
    let number_slices: &[i32; 5] = &[1,2,3,4,5];
    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    let book_slices: &[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];

    println!("Hello, world!");
    println!("Olá, {}!", name);
    println!("Number Array: {:?}", numbers);
    println!("Fruits Array: {:?}", fruits);
    println!("Human Tuple: {:?}", human);
    println!("Number Slice: {:?}", number_slices);
    println!("Animal Slice: {:?}", animal_slices);
    println!("Book Slice: {:?}", book_slices);

    // String vs String Slices (&str)
    let mut stone_cold: String = String::from("Hell, ");
    println!("Stone Cold Says: {}", stone_cold);
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // B- &str (Sring Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string;
    let slice2: &str = &string[0..5];
    println!("Slice Value: {}", slice);
    println!("Slice Value: {}", slice2);
}
