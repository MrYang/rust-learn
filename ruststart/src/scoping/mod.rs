extern crate rustcarte;

mod lifetimes;

pub fn s() {
    let a = rustcarte::add(3, 4);
    println!("add:{}", a);
}