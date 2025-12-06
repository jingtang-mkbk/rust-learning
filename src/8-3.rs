use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 不存在则插入，存在则不操作
    scores.entry(String::from("test")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(200);

    println!("{scores:#?}");

    for (key, val) in scores {
        println!("{}: {}", key, val);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
