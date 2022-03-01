fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // sumはイテレータの所有権を取得するので、sum呼び出しの後にv1_iterを使うことはできない
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }
}
