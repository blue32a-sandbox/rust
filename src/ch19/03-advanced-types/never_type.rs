// 関数barは返さない
fn bar() -> ! {
    panic!();
}

fn main() {
    // bar();

    let mut num = 0;

    loop {
        num += 1;

        let result = match num {
            5 => 5,

            // 整数型か文字列型か推測できない
            // _ => "hello",

            // continueは!値を持つので、任意の型に強制することができる
            _ => continue,
        };

        println!("{}", result);

        break;
    }
}
