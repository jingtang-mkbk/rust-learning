use std::usize;

fn main() {
    let mut str: String = String::from("aaa");
    str.push_str("bbb");

    println!("str: {}", str.len());

    let s: String = String::from("ccc");
    take_ownership(s);

    let n: u8 = 5;
    make_copy(n);
    println!("n: {}", n);

    let s1: String = give_ownership();
    println!("s1: {}", s1);
    let s2: String = String::from("ccc");
    let s3 = takes_and_gives_back(s2);
    println!("s3: {}", s3);

    let s4: String = String::from("hedsfhfdhdfsllo");
    let (s5, len) = calculate_length(s4);
    println!("s5: {}, len: {}", s5, len);
}

fn take_ownership(str: String) {
    println!("str: {}", str);
}

fn make_copy(n: u8) {
    println!("make_copy: {}", n);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(str: String) -> String {
    str
}

fn calculate_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}
