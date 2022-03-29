fn main() {
    let x = 1;

    // リテラルに対して直接パターンをマッチさせることができる
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
