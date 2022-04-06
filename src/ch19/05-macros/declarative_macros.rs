// vec!マクロを簡略化したもの
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#![allow(unused)]
fn main() {
    let v: Vec<u32> = vec![1, 2, 3];

    // 上記のvec!マクロ呼び出しは、次のようなコードに置き換えられる
    // {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }
}
