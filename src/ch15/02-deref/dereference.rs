fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);

    // error: no implementation for `{integer} == &{integer}`
    // 数値と数値への参照は型が違うから比較はできない
    // assert_eq!(5, y);

    // 参照解除演算子を使って、参照が指す値を追いかける
    assert_eq!(5, *y);
}
