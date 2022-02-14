fn main() {
    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{}", i);
    }

    let mut v2 = vec![100, 32, 57];
    for i in &mut v2 {
        // 可変参照が参照する値を変更するには間接参照演算子(*)が必要
        *i += 50;
    }
    println!("{:?}", v2);
}
