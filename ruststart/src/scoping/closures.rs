use std::thread;
use std::time::Duration;

pub fn c() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1 };
    let add_one_v4: fn(u32) -> u32 = |x: u32| x + 1;

    let c = |x| x;
    let c = c("x");
    println!("c:{}, a1:{}, a2:{}, a3:{}, a4:{}", c, add_one_v1(1), add_one_v2(1), add_one_v3(1), add_one_v4(1));

    let mut cacher = Cacher::new(|num| {
        println!("calc value");
        thread::sleep(Duration::from_millis(500));
        num
    });
    let v1 = cacher.value(5);
    let v2 = cacher.value(4);
    println!("v1:{}, v2:{}", v1, v2);
}

struct Cacher<T, U> where T: Fn(U) -> U {
    calc: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U> where T: Fn(U) -> U, U: Copy {
    fn new(calc: T) -> Cacher<T, U> {
        Cacher {
            calc,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> U {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calc)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        c();
    }
}