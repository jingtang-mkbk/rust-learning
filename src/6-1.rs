#[derive(Debug)]
enum IPAddrKind {
    V4,
    V6,
}

struct IPAddr {
    kind: IPAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IPAddr2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IPAddr2 {
    fn p(&self) {
        println!("asd");
    }
}

fn main() {
    let home = IPAddr {
        kind: IPAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };
    let loopback = IPAddr {
        kind: IPAddrKind::V6,
        addr: String::from("::1"),
    };

    println!("{:?} {}\n{:?} {}", home.kind, home.addr, loopback.kind, loopback.addr);

    let addr = IPAddr2::V4(127, 0, 0, 1);
    let addr2 = IPAddr2::V6(String::from("::1"));
    println!("{:?} {:?}", addr, addr2);
    addr.p();
}