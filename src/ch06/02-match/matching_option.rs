fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    println!("5 + 1 = {:?}", six);

    let none = plus_one(None);
    println!("None + 1 = {:?}", none);
}
