// Collection Types:
// Vectors - UTF8 - Hashmaps
fn main() {
    let _v:Vec<i32> = Vec::new();
    let mut _v:Vec<i32> = Vec::new();
    let mut _v:Vec<i32> = vec![1,2,3];
    _v.push(4);
    _v.push(5);
    _v.push(6);
    _v.push(7);

    println!("{:?}", _v);

    let _v2:Vec<i32> = vec![1,2,3,4,5];
    let third:&i32 = &_v2[2]; // Direct indexing
    println!("The third element is {}", third);

    let third:Option<&i32> = _v2.get(2);
    match third {
        Some(third) => println!("The third element from the GET is {}", third),
        None => println!("There is no third element"),
    }
}
