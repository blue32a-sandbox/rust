fn main() {
    let some_option_value: Option<i32> = None;

    // let文は反論できないパターン（irrefutable）しか受け付けない
    // error: refutable pattern in local binding: `None` not covered
    // let Some(x) = some_option_value;

    // 反論できるパターン（refutable）はif letを使用する
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // 常にマッチするパターンをif letに与えると警告が出る
    // warning: irrefutable `if let` pattern
    if let x = 5 {
        println!("{}", x);
    };
}
