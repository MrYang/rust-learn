pub fn strings() {
    // String 被存储为由字节组成的 vector（Vec<u8>），String 是堆分配的，可增长的，且不是零结尾的（null terminated）。
    // &str 是一个指向有效 UTF-8 序列的切片（&[u8]），可用来查看 String 的内容，就如同 &[T] 是 Vec<T> 的全部或部分引用。

    let s: &'static str = "hello world";
    println!("{}", s);

    for word in s.split_whitespace().rev() {
        println!("{}", word);
    }

    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();

    let mut s = String::new();
    for c in chars {
        s.push(c);
        s.push_str(", ");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = s.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strings() {
        strings();
    }
}
