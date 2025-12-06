pub trait Area<T> {
    fn area(&self) -> T;
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Area<u32> for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub struct Circle {
    pub radius: u32,
}

impl Area<f32> for Circle {
    fn area(&self) -> f32 {
        (self.radius * self.radius) as f32 * std::f32::consts::PI
    }
}

pub struct Trapezium {
    pub base1: u32,
    pub base2: u32,
    pub height: u32,
}

impl Area<u32> for Trapezium {
    fn area(&self) -> u32 {
        (self.base1 + self.base2) * self.height / 2
    }
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
