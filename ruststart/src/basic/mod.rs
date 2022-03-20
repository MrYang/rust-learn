pub mod types;
pub mod control;
pub mod generics;
pub mod traits;
pub mod vector;
pub mod errors;

pub fn pub_method(){
    println!("const pi:{}", types::CONST_PI);
    types::basic_types()
}