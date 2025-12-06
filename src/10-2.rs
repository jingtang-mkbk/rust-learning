use project_rust::{Area, Circle, Rectangle, Trapezium};

fn print_area(item: &impl Area<u32>) {
    println!("area: {}", item.area());
}

fn print_area2<T: Area<u32>>(item: &T) {
    println!("area: {}", item.area());
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    // println!("rect area: {}", rect.area());
    print_area(&rect);
    print_area2(&rect);

    let circle = Circle { radius: 10 };
    println!("circle area: {}", circle.area());

    let trapezium = Trapezium {
        base1: 10,
        base2: 20,
        height: 30,
    };
    println!("trapezium area: {}", trapezium.area());
}
