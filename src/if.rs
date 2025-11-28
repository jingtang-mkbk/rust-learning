fn main() {
    let a = 5;

    if a < 6 {
        println!("less");
    } else {
        println!("greater");
    }

    let b: bool = true;
    let c: u8 = if b { 6 } else { 7 };
    println!("c: {c}"); 
}