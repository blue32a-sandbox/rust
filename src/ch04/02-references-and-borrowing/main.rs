fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // sは文字列への参照です
    s.len()
} // ここで、sはスコープ外になります。 しかし、それが参照するものの所有権を持っていないため、何も起こりません。
