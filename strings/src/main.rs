fn main() {
    // String are stored ad a collection of UTF-8 encoded bytes
    // Creation
    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");

    //grow in size
    let mut s5 = String::from("foo");
    s5.push_str("bar");
    s5.push('!');

    let s6 = String::from("Hello ");
    let s7 = String::from("world!");
    let s8: String = s6 + &s7;

    // same but as a macro
    // let s8 = format!("{}{}", s6, s7);
}
