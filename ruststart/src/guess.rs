use std::cmp::Ordering;
use std::io;

pub fn g() {
    println!("guess number");

    let secret_number = rand::random::<u8>();

    println!("secret_number:{}", secret_number);

    loop {
        println!("请输入你的数字");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("异常");

        //let g: u8 = guess.trim().parse().expect("解析数字错误");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_guess() {
        g();
    }
}
