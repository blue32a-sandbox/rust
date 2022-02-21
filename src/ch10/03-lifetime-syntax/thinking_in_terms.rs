fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 戻り値のラフタイムと関係ないので、yにはライフタイムパラメータを指定していない
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // 戻り値のライフタイムがパラメータのライフタイムと全く関係ないためエラーになる
    result.as_str()
}
