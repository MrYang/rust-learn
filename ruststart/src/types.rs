pub const CONST_PI: f64 = 3.1415;

const S: &str = "pri s";

pub fn t() {
    let mut a = 6;
    let emoji = '😻';
    println!("a:{}", a);

    a = 7;
    let t = (500, 5.43, "abc", 98_222, 0xfff);
    println!("a:{}, S:{}, emoji:{}, tuple:{:?}, i32:{}, f64:{}, str:{}", a, S, emoji, t, t.0, t.1, t.2);

    let a = [1, 2, 3];
    let a_2 = [1; 5];
    let a_3: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a:{:?}, a_2:{:?}, a_3:{:?}", a, a_2, a_3);

    let s = "abcdefghijklimn";
    let s_2 = String::from("1234567890");
    let sli_1 = &s[0..5];
    let sli_2 = &s_2[5..=6];
    let sli_3 = &s_2[..=6];

    println!("sli_1:{}, sli_2:{}, sli_3:{}", sli_1, sli_2, sli_3);

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
        width:30,
        height:30,
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
    fn area(&self) -> u32 {
        self.width * self.height
    }
}