fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangleは文字列への参照を返します
    let s = String::from("hello");

    &s // 文字列への参照を返します。
} // ここで、sはスコープ外になり、削除されます。その記憶は消えます。危険！
