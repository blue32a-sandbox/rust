fn main() {
    // マッチガードは、パターンだけでは表現できないような複雑なアイデアを表現できる
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 外部変数との等価性テスト
    let x = Some(5);
    let y = 10;
    // let y = 5; // ２つ目にマッチする

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // 複数のパターンをマッチガードで統合する
    let x = 4;
    let y = false;
    // let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
