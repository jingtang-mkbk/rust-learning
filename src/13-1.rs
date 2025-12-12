use std::thread;

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn add_one_v1(x: u8) -> u8 {
    x + 1
}

fn main() {
    let add_one_v2 = |x: u32| -> u32 {x + 1};
    let add_one_v3 = |x| {x + 1};
    let add_one_v4 = |x| x + 1;

    println!("v1:{} v2:{} v3:{} v4:{}", add_one_v1(1), add_one_v2(2), add_one_v3(3), add_one_v4(4));

    let list = vec![1, 2, 3];
    println!("Before definning closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut l = [
        Rect { width: 3, height: 5 },
        Rect { width: 10, height: 1 },
        Rect { width: 7, height: 12 },
    ];
    l.sort_by_key(|r| r.height);
    println!("l: {l:#?}");
}