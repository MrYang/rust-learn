use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

pub fn files() {
    let path = Path::new("hello.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    io::BufReader::new(file).lines().for_each(|line| {
        println!("{}", line.unwrap());
    });

    let mut s = String::new();
    match File::open(&path).unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_files() {
        files();
    }
}
