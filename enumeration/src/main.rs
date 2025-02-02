#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn coinage() {
    let mut total = 0;
    let value = value_in_cents(Coin::Penny);
    total += value;
    println!("total value: {}", total);
    let value = value_in_cents(Coin::Dime);
    total += value;
    println!("total value: {}", total);
    let value = value_in_cents(Coin::Quarter(UsState::Alabama));
    total += value;
    println!("total value: {}", total);
}

fn if_some_add() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


fn main() {
    coinage();
    if_some_add();
}
