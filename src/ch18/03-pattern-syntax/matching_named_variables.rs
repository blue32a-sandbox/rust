fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // 最初に宣言したyではない
        _ => println!("Default case, x = {:?}", x), // xは外側のxのまま
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
