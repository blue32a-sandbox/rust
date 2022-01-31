fn main() {
    let s1 = gives_ownership(); // gives_ownershipはその戻り値をs1に移動します
    println!("s1 {}", s1);

    let s2 = String::from("hello"); // s2がスコープに入る
    println!("s2 {}", s2);

    let s3 = takes_and_gives_back(s2); // s2はtakes_and_gives_backに移動され、takes_and_gives_backは戻り値もs3に移動します
    // println!("s2 {}", s2); // error: borrow of moved value: `s2`
    println!("s3 {}", s3);

    let s5 = String::from("hello");
    let (s6, len) = calculate_length(s5);
    // println!("s5 {}", s5); // error: borrow of moved value: `s5`
    println!("The length of '{}' is {}.", s6, len);
}

fn gives_ownership() -> String { // give_ownershipは、戻り値をそれを呼び出す関数に移動します
    let some_string = String::from("yours"); // some_stringがスコープに入る

    some_string // some_stringが返され、呼び出し元の関数に移動します
}

fn takes_and_gives_back(a_string: String) -> String {// a_stringがスコープに入る
    a_string // a_stringが返され、呼び出し元の関数に移動します
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length) // タプルを使用して複数の値を返すことができます
}
