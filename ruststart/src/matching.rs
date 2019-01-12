pub fn m() {

    let home = IpAddr {
      kind: IpAddrKind::V4(127, 0, 0, 1),
      addr: String::from("127.0.0.1"),
    };

    let lookback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        addr: String::from("::1"),
    };

    println!("home:{:#?}, lookback:{:#?}, p:{}", home, lookback, home.kind.p());

    let v = match home.kind {
        IpAddrKind::V4(127, 0, 0, 2) => 4,
        _ => {
            println!("default");
            0
        },
    };

    println!("v:{}", v);
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn p(&self) -> String {
        format!("p fn")
    }
}