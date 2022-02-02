fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("{}, {}, {}", black.0, black.1, black.2);
    let origin = Point(0, 0, 0);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);
}
