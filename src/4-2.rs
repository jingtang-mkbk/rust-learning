fn main() {
    let mut s1: String = String::from("hello");
    let len: usize = calculate_length(&mut s1);
    println!("len: {}, s1: {}", len, s1);
}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}
