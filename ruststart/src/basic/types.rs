pub const CONST_PI: f64 = 3.1415;

const S: &str = "pri s";

pub fn basic_types() {
    let mut i = 6;
    let emoji = '😻';
    let b = false;
    println!("i:{}, b:{}", i, b);

    i = 7;
    println!("a:{}, S:{}, emoji:{}", i, S, emoji);
}

pub fn tuple_types() {
    let t = (500, 5.43, "abc", 98_222, 0xfff);
    println!(
        "S:{}, tuple:{:?}, i32:{}, f64:{}, str:{}",
        S, t, t.0, t.1, t.2
    );
}

pub fn array_types() {
    let a = [1, 2, 3];
    let a_2 = [1; 5];
    let a_3: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a:{:?}, a_2:{:?}, a_3:{:?}", a, a_2, a_3);
}

pub fn str_types() {
    let s = "abcdefghijklimn";
    let s_2 = String::from("1234567890");
    let sli_1 = &s[0..5];
    let sli_2 = &s_2[5..=6];
    let sli_3 = &s_2[..=6];

    println!("sli_1:{}, sli_2:{}, sli_3:{}", sli_1, sli_2, sli_3);
}

pub fn enum_types() {
    let home = IpAddr {
        kind: IpAddrKind::V4(127, 0, 0, 1),
        addr: String::from("127.0.0.1"),
    };

    let lookback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1")),
        addr: String::from("::1"),
    };

    println!(
        "home:{:#?}, lookback:{:#?}, p:{}",
        home,
        lookback,
        home.kind.p()
    );

    let v = match home.kind {
        IpAddrKind::V4(127, 0, 0, 2) => 4,
        _ => {
            println!("default");
            0
        }
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

pub fn struct_types() {
    let user = User {
        username: String::from("username"),
        email: String::from("email"),
        sign_in_count: 0,
        active: false,
    };

    let user1 = build_user(String::from("username"), String::from("email"));

    // user2 borrow user, 所以user 不可用了
    let user2 = User {
        username: String::from("user2"),
        ..user
    };

    let user3 = User {
        username: String::from("user3"),
        email: String::from("email3"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    println!("user1:{:?}, user2:{:?}, user3:{:#?}", user1, user2, user3);

    let rect = Rectangle {
        width: 30,
        height: 30,
    };
    let area = area(&rect);
    println!("area:{}, {}", area, rect.area());
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

use std::fmt;
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "username:{}, email:{}, active:{}", self.username, self.email, self.active)
    }
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0,
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    // 方法
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

type By = u16;

pub fn convert() {
    let d = 68.4;
    let i = d as u32;
    let h = i as By;
    let u = i as u8;
    let c = u as char;

    println!("d:{}, h:{}, i:{}, u:{}, c:{}", d, h, i, u, c);
}
