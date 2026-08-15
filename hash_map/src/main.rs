use std::{collections::HashMap};

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name:String = String::from("Blue");
    let _score:i32 = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Green")).or_insert(40);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // {"hello", "world", "wonderful", "world"}
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
