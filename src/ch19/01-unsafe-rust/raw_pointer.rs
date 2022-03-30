fn main() {
    let mut num = 5;

    // 生ポインタの型にキャストすることで生ポインタを作成
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 安全でないブロックを必要とする生ポインタに対して、
    // 参照解除演算子を使用する。
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
