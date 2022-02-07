fn main() {
    let some_number = Some(5);
    println!("{:?}", some_number);

    let some_string = Some("a string");
    println!("{:?}", some_string);

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);
}
