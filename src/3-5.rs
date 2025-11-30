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

    let mut count: u8 = 0;
    loop {
        if count <= 10 {
            println!("count: {count}");
            count += 1;
        } else {
            break;
        }
    }

    let mut count2 = 0;
    let result = loop {
        count2 += 1;
        if count2 == 10 {
            break count2 * 2;
        }
    };
    println!("count2: {count2}");

    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("item: {element}");
    }

    for num in (10..=15).rev() {
        println!("{num}");
    }

    for ele in [9, 8, 7, 6, 5] {
        println!("{ele}");
    }
}
