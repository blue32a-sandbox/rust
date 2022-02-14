fn main() {
    // 空の文字列を新規に作成する
    let mut s = String::new();
    println!("{}", s);

    // to_string メソッドを使用して文字列リテラルから文字列を作成する
    let data = "inital contents";
    let s = data.to_string();
    println!("{}", s);

    // このメソッドは、リテラルに対しても直接機能します
    let s = "inital contents".to_string();
    println!("{}", s);

    // String::from関数を使って文字列リテラルから文字列を生成する
    let s = String::from("inital contents");
    println!("{}", s);

    // 文字列はUTF-8でエンコードされているので、適切にエンコードされたデータを含めることができる
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
