use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create file error: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", e),
        },
    };

    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("read file error");
    println!("{}", buf);

    let username = read_username_from_file().expect("read username error");
    println!("{}", username);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}
