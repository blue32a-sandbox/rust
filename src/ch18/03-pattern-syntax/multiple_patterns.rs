fn main() {
    let x = 1;

    match x {
        // |構文で複数のパターンにマッチさせることができる
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
