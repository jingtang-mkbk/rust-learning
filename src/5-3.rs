#[derive(Debug)]
struct Rect {
    witdh: u32,
    height: u32
}

impl Rect {
    // fn area(self: &Self) -> u32 {       &self 实际上是 self: &Self 的缩写
    fn area(&self) -> u32 {
        self.witdh * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.witdh > other.witdh && self.height > other.height
    }
}

fn main() {
    let rect = Rect {
        witdh: 10,
        height: 60
    };

    println!("{:#?}", rect);
    println!("rect area: {}", rect.area());

    let rect2 = Rect {
        witdh: 10,
        height: 10
    };
    let rect3 = Rect {
        witdh: 11,
        height: 11
    };
    println!("rect3 can hold rect2: {}", rect3.can_hold(&rect2));
}