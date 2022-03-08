fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);

    // error: no implementation for `{integer} == Box<{integer}>`
    // assert_eq!(5, y);

    // Boxも参照解除演算子を使用することができる
    assert_eq!(5, *y);
}
