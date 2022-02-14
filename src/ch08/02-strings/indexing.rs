fn main() {
    // 4バイト
    let hello = String::from("Hola");
    println!("{}", hello);

    // 12バイトではなく24バイト
    let hello = String::from("Здравствуйте");
    println!("{}", hello);

    // Rustではインデックスで参照して文字列内の個々の文字にアクセスできない
    // let answer = &hello[0]; // error: the type `String` cannot be indexed by `{integer}`
}
