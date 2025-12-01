#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 40
    };
    println!("area: {}", rect_area(&rect));
    println!("rect is {rect:#?}");
}

fn rect_area(rect: &Rect) -> u32 {
    rect.width * rect.height
}