#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct Rect<T> {
    w: T,
    h: T,
}

impl<T> Rect<T> {
    fn getW(&self) -> &T {
        &self.w
    }
}

fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut max: T = arr[0];
    for item in arr {
        if *item > max {
            max = *item;
        }
    }
    max
}

fn main() {
    let mut v: Vec<u32> = vec![1, 2, 3];
    v.push(4);
    let max = largest(&v);
    println!("max: {}", max);

    let mut str: Vec<&str> = vec!["a", "b", "c"];
    str.push("d");
    let max2 = largest(&str);
    println!("{:?}", max2);

    let p: Point<u8> = Point { x: 10, y: 20 };
    println!("{p:#?}\n{} {}", p.x, p.y);

    let rect = Rect { w: 100, h: 200 };
    println!("{rect:#?}");
    println!("w: {}", rect.getW());
}
