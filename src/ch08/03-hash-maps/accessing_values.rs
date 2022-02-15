fn main() {
    // ハッシュマップに格納された青チームのスコアにアクセスする
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    // getはOption<&V>を返す
    let score = scores.get(&team_name);

    println!("{} team score {:?}", team_name, score);

    // forループを使って反復処理することができる
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
