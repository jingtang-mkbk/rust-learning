#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     if let Coin::Quarter(state) = coin {
//         if (state.existed_in(1900)) {
//             Some(format!("{state:?} is pretty old, for America!"))
//         } else {
//             Some(format!("{state:?} is relatively new."))
//         }
//     } else {
//         None
//     }
// }

// fn describe_state_quarter(coin: Coin) -> Option<String> {
//     let state = if let Coin::Quarter(state) = coin {
//         state
//     } else {
//         return None;
//     };
//     if state.existed_in(1900) {
//         Some(format!("{state:?} is pretty old, from America!"))
//     } else {
//         Some(format!("{state:?} is relatively new."))
//     }
// }

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, from America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn main() {
    // let c = Coin::Quarter(UsState::Alaska);
    let c = Coin::Penny;
    let d = describe_state_quarter(c);
    println!("{d:?}");
}