fn main() {
    // consider giving `guess` a type
    // let guess = "42".parse().expect("Not a number!");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("4 * 30 = {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("2 / 3 = {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);

    // boolean
    let t = true;
    println!("t = {}", t);
    let f: bool = false; // with explicit type annotation
    println!("f = {}", f);

    // character
    let c = 'z';
    println!("c = {}", c);
    let z = 'ℤ';
    println!("z = {}", z);
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {}", heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("({}, {}, {})", five_hundred, six_point_four, one);

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);
}
