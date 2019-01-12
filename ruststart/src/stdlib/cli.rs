use std::env;
use std::fs;

pub fn c() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let exe = env::current_exe().expect("读取当去exe 目录错误");
    println!(":{}", exe.display());
}