fn main() {
    let mut s: String = String::from("hel3lo worl1111d");
    let word: &str = first_word(&s);
    println!("word: {}", word);
    s.clear();

    let s2 = String::from("hello world");
    let slice = &s2[3..s2.len()];
    println!("slice: {}", slice);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
