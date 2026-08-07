fn main() {
    let mut counter = 0;

    let result = loop {

        println!("The counter is {}", counter);
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
    println!("-------------------");

    // Loop Labels to Disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remanining = 10;

        loop {
            println!("remanining = {remanining}");
            if remanining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remanining -= 1;
        }
        count += 1;
    }
    println!("-------------------");


    // While Loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("Finish!");
    println!("-------------------");

    // Looping Through a collection with for loop
    let a = [1, 2, 3, 4, 5, 6];
    let b = ["a", "b", "c", "d", "e"];
    for element in a {
        println!("{element}");
    }

    for element in b {
        println!("{element}");
    }
}
