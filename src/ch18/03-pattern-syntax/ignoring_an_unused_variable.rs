fn main() {
    // 警告が出ない
    let _x = 5;

    // こちらは未使用なので警告が表示される
    // warning: unused variable: `y`
    let y = 10;

    let s = Some(String::from("Hello!"));

    // 値が_sに束縛される
    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
