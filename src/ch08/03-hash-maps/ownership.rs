fn main() {
    // キーと値が挿入されるとハッシュマップに所有されることを示す

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // 変数 field_name と field_value は、ハッシュマップに移動する
    map.insert(field_name, field_value);
    println!("{:?}", map);

    // error: borrow of moved value: `field_name`
    // error: borrow of moved value: `field_value`
    // println!("{}, {}", field_name, field_value);
}
