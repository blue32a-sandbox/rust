fn main() {
    // push_strメソッドで文字列スライスをStringに追加する
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    // 文字列スライスの内容をStringに追記した後に使用する
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // s2の所有権は取得しない
    println!("s2 is {}", s2);

    // プッシュを使用して String 値に 1 文字を追加する
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // +演算子を使用して2つの String値を新しいString値に結合する
    let hello = String::from("Hello, ");
    let world = String::from("world!");
    // helloの所有権を取得してworldのコピーを追加し、結果の所有権が返される
    let helloworld = hello + &world; // helloは使用できなくなる
    println!("{}", helloworld);
    // println!("{}", hello); // error: borrow of moved value: `hello`
    println!("{}", world);
}
