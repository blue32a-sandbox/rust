fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // value borrowed here after move
}
