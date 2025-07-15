pub fn if_control() {
    let num = 5;

    if num % 2 == 0 {
        println!("num 可以被2整除");
    } else if num % 3 == 0 {
        println!("num 可以被3整除");
    } else {
        println!("num 不被2,3整除");
    }

    let n = if num % 2 == 0 { num } else { num - 1 };
    println!("n:{}", n);
}

pub fn loop_control() {
    for i in 1..=5 {
        if i == 2 {
            continue;
        }
        println!("{}", i)
    }

    let a = [1, 2, 3, 4];
    for (i, v) in a.iter().enumerate() {
        println!("idx:{}, v:{}", i, v);
    }

    let mut n = 0;
    while n < 5 {
        println!("{}", n);
        n = n + 1;
    }

    loop {
        println!("{}", n);
        if n > 10 {
            break;
        }
        n = n + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_control() {
        if_control();
    }

    #[test]
    fn test_loop_control() {
        loop_control();
    }
}
