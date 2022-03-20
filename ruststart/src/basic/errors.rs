use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

fn error_handler1() {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        };

        let f1 = File::open("hello.txt").unwrap();
        let f2 = File::open("hello.txt").expect("failed open hello.txt");

        println!("{:#?}, {:#?}, {:#?}", f, f1, f2);
}

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}