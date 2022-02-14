fn main() {
    // 本当にインデックスを使用する必要がある場合は、具体的な範囲をする必要がある。
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // panicked
    // let s = &hello[0..1];
}
