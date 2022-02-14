fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // panicked
    let does_not_exist = v.get(100); // return None
    println!("{:?}", does_not_exist);

    let mut v2 = vec![1, 2, 3, 4, 5];
    let first = &v2[0];
    // v2.push(6); // エラー：同じスコープ内に可変参照と不変参照を含めることはできない
    println!("The first element is: {}", first);
    v2.push(6); // こちらはOK
    println!("{:?}", v2);
}
