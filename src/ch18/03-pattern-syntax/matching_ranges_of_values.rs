fn main() {
    // ..=構文は値を含む範囲にマッチさせることができる
    // char型と数値型のみ許可される（Rustが範囲が空かどうか確認できるのはこの２つ）

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
