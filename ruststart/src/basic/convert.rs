#[derive(Debug)]
struct Number {
    num: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { num: item }
    }
}

pub fn from() {
    let s1 = String::from("hello");
    println!("{}", s1);

    let n1: Number = 1.into();
    let n2: Number = Number::from(2);
    println!("{:?}, {:?}", n1, n2);
}

pub fn parse() {
    let parsed = "42".parse::<i32>().unwrap();
    println!("{}", parsed);

    let parsed = "42".parse::<i32>();
    match parsed {
        Ok(i) => println!("parsed: {}", i),
        Err(_) => println!("parse error"),
    }

    let parsed = "42".parse::<i32>().expect("parse error");
    println!("parsed: {}", parsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        from();
    }

    #[test]
    fn test_parse() {
        parse();
    }
}
