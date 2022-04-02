fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // クロージャと名前付き関数のどちらを使うかの例
    let list_of_numbers = vec![1, 2, 3];

    // クロージャー
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
    println!("list of strings (use closure): {:?}", list_of_strings);

    // 名前付き関数
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    println!("list of strings (use name a function): {:?}", list_of_strings);

    // タプル構造体やタプル構造体のenumバリアントの実装の詳細を利用した便利なパターン
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    // Status::Valueのイニシャライザー関数を使用
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("list of statuses: {:?}", list_of_statuses);
}
