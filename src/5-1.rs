struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(u8, u8, u8, f32);

fn main() {
    let u1 = User {
        active: true,
        user_name: String::from("zhangsan"),
        email: String::from("zhangsan@163.com"),
        sign_in_count: 1,
    };

    println!("active: {}\nuser_name: {}\nemail: {}\nsign_in_count: {}\n", u1.active, u1.user_name, u1.email, u1.sign_in_count);

    let u2 = User {
        user_name: String::from("lisi"),
        email: String::from("lisi@163.com"),
        ..u1
    };
    println!("active: {}\nuser_name: {}\nemail: {}\nsign_in_count: {}\n", u2.active, u2.user_name, u2.email, u2.sign_in_count);

    let u3 = build_user(String::from("wangmazi"), String::from("wangmazi@163.com"));
    println!("active: {}\nuser_name: {}\nemail: {}\nsign_in_count: {}\n", u3.active, u3.user_name, u3.email, u3.sign_in_count);

    let color = Color(166, 201, 68, 0.5);
    println!("r: {} g: {} b: {} a: {}", color.0, color.1, color.2, color.3);
    let Color(r, g, b, a) = color;
    println!("r: {} g: {} b: {} a: {}", r, g, b, a);
}

fn build_user(user_name: String, email: String) -> User{
    User {
        active: false,
        user_name,
        email,
        sign_in_count: 2
    }
}