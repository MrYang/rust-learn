pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(Debug)]
struct Point<T, U>
where
    T: Copy,
    U: Copy,
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
where
    T: Copy,
    U: Copy,
{
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32, f64> {
    fn area(&self) -> f64 {
        return self.x as f64 * self.y;
    }
}

pub fn area() {
    let p = Point { x: 1, y: 1.1 };
    println!("{:?}", p);
}
