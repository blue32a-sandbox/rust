fn main() {
    let s = String::from("hello");
    takes_ownership(s); // sの値は関数に移動します...
    // println!("{}", s); // ...したがってここでは無効になります

    let x = 5;
    makes_copy(x); // xは関数に移動しますが、i32はコピーなので、後でxを使用しても問題ありません。
    println!("{}", x);
} // ここで、xはスコープ外になり、次にsになります。 しかし、sの値が移動されたため、特別なことは何も起こりません。

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // ここでは、some_stringがスコープ外になり、 `drop`が呼び出されます。 バッキングメモリが解放されます。

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // ここで、some_integerはスコープ外になります。 特別なことは何もありません。
