fn main() {
////スライスの基礎
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello is '{}',world is {}.", hello, world);



    let my_string = String::from("hello world");

    // first_wordは`String`のスライスに対して機能する
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_wordは文字列リテラルのスライスに対して機能する
    let word2 = first_word(&my_string_literal[..]);

    // 文字列リテラルは「それ自体すでに文字列スライス」なので、
    // スライス記法なしでも機能するのだ！
    let word2 = first_word(my_string_literal);

    println!("word2 is {}.", word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}