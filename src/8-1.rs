enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<u8> = vec![11, 22, 33];
    let mut v2: Vec<u8> = Vec::new();
    v2.push(1);
    v2.push(2);
    println!("{} {:?} {}", v2[0], v2.get(1), &v2[1]);

    v.push(44);
    let first = v[0];
    v.push(55);
    println!("{}\n", first);

    for i in &mut v {
        *i += 10;
        println!("{} {}", i, *i);
    }

    let row: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(100),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("aaa")),
    ];
    for i in row {
        match i {
            SpreadSheetCell::Int(i) => println!("int: {}", i),
            SpreadSheetCell::Float(f) => println!("float: {}", f),
            SpreadSheetCell::Text(s) => println!("text: {}", s),
        }
    }
}
