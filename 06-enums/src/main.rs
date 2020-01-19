#![allow(unused_variables,dead_code)]

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_u8_value = 3;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // _ = default; () = no-op
    }

    let coin = Coin::Dime;
    println!("Value in cents is {}", value_in_cents(coin));

    let mut count = 0;

    // two different syntaxes to write a match block with only one or two conditions:

    let coins = [
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Alaska),
        Coin::Penny,
    ];
    
    for coin in coins.iter() {
        // syntax #1
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }
        
    let coins = [
        Coin::Quarter(UsState::Missouri),
        Coin::Dime
    ];
    
    for coin in coins.iter() {
        // syntax #2
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }

    println!("Total non-quarter count: {}", count);
}

// enums can have state
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // state can be complex
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // enums are similar to structs
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Missouri,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
