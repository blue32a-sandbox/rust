// 関数ポインタfnを戻り値の型として使うことは出来ない
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// トレイトオブジェクトを使用する
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {
    let add_one = returns_closure();
    println!("{}", add_one(5));
}
