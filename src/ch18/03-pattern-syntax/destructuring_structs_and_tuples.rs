fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    // 複雑な方法で再構築のパターンを組み合わせたり、入れ子にしたりすることができる
    let ((fee, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10});
    println!("fee {}, inches {}, x {}, y {}", fee, inches, x, y);
}
