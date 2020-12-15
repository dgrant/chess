fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let s2 = String::from("hello world");

    let hello = &s2[0..5];
    let world = &s2[6..11];
    println!("hello={}", hello);
    println!("world={}", world);
}
