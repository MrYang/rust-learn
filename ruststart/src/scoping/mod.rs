extern crate rustcarte;

mod closures;
mod lifetimes;

pub fn s() {
    let a = rustcarte::add(3, 4);
    println!("add:{}", a);
}
