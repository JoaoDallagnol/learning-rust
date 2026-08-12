enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn some_function() {
        println!("Let's get Rusty!");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    addres: String
}

fn main() {
    let _localhost = IpAddrKind::V4(127,0,0,1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    //-----------
    // Using enhance enums
    enum IpAddrEnhance {
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let _home_enhance = IpAddrEnhance::V4(127,0,0,1);
    let _loopback_enhance = IpAddrEnhance::V6(String::from("::1"));

    println!("-------------------------");
    println!("Enum Option");
    let x: i8 = 5;
    let y: Option<i8> = Some(5); // or = None it still works

    let sum = x + y.unwrap_or(0);
    println!("Sum of x +  y is {}", sum);

    println!("-------------------------");
    value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("-------------------------");
    println!("Match with Enum Option");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

}

fn route(_ip_kind: IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

    // Use when there is multiples cases for match but 
    // just one some of then the rest is the same as None
    
    // match x {
    //     Some(i) => Some(i + 1),
    //     _ => None
    // }
}
