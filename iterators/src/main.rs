fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // iter() borrows each value.
    // value is &i32 here, so v1 is still valid after this for.
    for value in v1_iter {
        println!("Borrowed: {}", value);
    }

    println!("v1 is still available: {:?}", v1);

    // into_iter() on a vector moves each value out.
    // value is i32 here, and v1 cannot be used after this point.
    for value in v1.into_iter() {
        println!("Moved: {}", value);
    }

    let mut v2 = vec![10, 20, 30];

    // iter_mut() borrows each value mutably.
    // value is &mut i32 here, so you can change the elements in place.
    for value in v2.iter_mut() {
        *value += 1;
    }

    println!("v2 after iter_mut: {:?}", v2);

    let v3 = vec![100, 200, 300];
    let mut iterator = v3.iter();

    // next() advances the iterator one item at a time.
    println!("first next: {:?}", iterator.next());
    println!("second next: {:?}", iterator.next());
    println!("third next: {:?}", iterator.next());
    println!("fourth next: {:?}", iterator.next());

    let v4 = vec![1, 2, 3, 4, 5, 6];

    // Iterator adapters like map and filter are lazy.
    // They describe work, but do not run until a consumer is called.
    let even_numbers_doubled: Vec<i32> = v4
        .iter()
        .filter(|number| *number % 2 == 0)
        .map(|number| number * 2)
        .collect();

    println!("even numbers doubled: {:?}", even_numbers_doubled);

    let v5 = vec![1, 2, 3];

    // sum() consumes the iterator and returns one final value.
    let total: i32 = v5.iter().sum();

    println!("sum: {}", total);

    let names = vec!["Ana", "Joao", "Maria"];

    // enumerate() gives the index and the value while iterating.
    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name);
    }
}
