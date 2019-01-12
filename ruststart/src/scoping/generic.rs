
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        return Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn g() {
    let p1 = Point {x:5, y:10};
    let p2 = Point {
        x: 5.0,
        y: 10.0,
    };

    println!("p1:{:?}, p2:{:?}", p1, p2);

    let p3 = p1.mixup(p2);
    println!("p3:{:?}", p3);

    let number_list = vec![1, 40, 10, 5, 14];
    let result = largest(&number_list);
    println!("result:{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("result:{}", result);
}