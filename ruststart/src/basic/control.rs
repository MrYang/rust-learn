pub fn if_control() {
    let num = 5;

    if num % 2 == 0 {
        println!("num 可以被2整除");
    } else if num % 3 == 0 {
        println!("num 可以被3整除");
    } else {
        println!("num 不被2,3整除");
    }

    let n = if num % 2 == 0 { num } else { num - 1 };
    println!("n:{}", n);
}


pub fn for_control() {
    
}