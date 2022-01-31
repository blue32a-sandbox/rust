fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // これにより文字列が空になり、「」と等しくなります。

    // ここでは、wordの値はまだ5ですが、値5を意味のある形で使用できる文字列はこれ以上ありません。
    // 単語が完全に無効になりました！
    println!("{}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
