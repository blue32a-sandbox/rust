fn main() {
    // 新しいハッシュマップを作成し、いくつかのキーと値を挿入する
    use std::collections::HashMap;
    let mut scores = HashMap::new();

    // すべてのキーは同じ型、すべての値は同じ型
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // チームのリストとスコアのリストからハッシュマップを作成する
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // キーと値の型のパラメータにはアンダースコアを使用すると、
    // Rustはベクトル内のデータの型からハッシュマップが含む型を推測することができる。
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores);
}
