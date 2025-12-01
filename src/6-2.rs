#[derive(Debug)]
enum UsStata {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickek,
    Dime,
    Quarter(UsStata)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickek => 5,
        Coin::Dime => 10,
        Coin:: Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn main() {
    let c = Coin::Quarter(UsStata::Alabama);
    let v = value_in_cents(c);
    println!("{v}");

    let p = plus_one(Some(5));
    println!("{:?}", p);
}