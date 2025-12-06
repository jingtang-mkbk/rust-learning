use project_rust::first_word;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let str = String::from("value");
    let str2 = "asfdsfas";

    let res = longest(str.as_str(), str2);
    println!("{}", res);

    let str3 = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = str3.split('.').next().unwrap();
    println!("{}", first_sentence);

    let first_word = first_word(str3.as_str());
    println!("{}", first_word);
}
