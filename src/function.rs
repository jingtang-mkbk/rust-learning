

fn main() {
    let x = {
        let y = 1;
        y + 2
    };
    println!("{x}");

    another_function(5, 'h');
    println!("five function {}", five());
    println!("six function {}", six());
    
    match plus_one(254) {
        Some(v) => println!("plus_one function {}", v),
        None => println!("Error"),
    }
}

fn another_function(x: u8, str: char) {
    println!("Another function {x}{str}.");
}

fn five() -> u8 {
    5
}

fn six() -> u8 {
    return 6;
}

fn plus_one(num: u8) -> Option<u8> {
    num.checked_add(1)
}