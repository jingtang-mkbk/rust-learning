fn main() {
    let data = "aabbcc";
    let mut str = data.to_string();
    let s = "aabbcc".to_string();
    let s2 = String::from("eeffgg");
    str.push_str(" string");
    str.push('!');

    println!("{}", str);

    let s3 = s + &s2;
    println!("{}", s3);

    let s4 = String::from("aa");
    let s5 = String::from("bb");
    let s6 = String::from("cc");
    let s7 = format!("{s4}-{s5}-{s6}");
    println!("{}", s7);
    println!("len: {}", s7.len());
}
