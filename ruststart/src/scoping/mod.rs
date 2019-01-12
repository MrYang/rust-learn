extern crate rustcarte;

mod generic;
mod traits;
mod lifetimes;

//use rustcarte;

pub fn s() {
    generic::g();
    traits::t();
    let a = rustcarte::add(3, 4);
    println!("add:{}", a);
}