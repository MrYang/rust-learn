
type By = u16;

pub fn c() {
    let d = 68.4;

    let i = d as u32;

    let b = i as By;

    let u = i as u8;

    let c = u as char;

    println!("d:{}, b:{}, i:{}, u:{}, c:{}", d, b, i, u, c);
}