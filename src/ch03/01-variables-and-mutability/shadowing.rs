fn main() {
    let x = 5;

    let x = x + 1; // shadowed

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces len : {}", spaces);

    // let mut spaces = "   ";
    // spaces = spaces.len(); // expected `&str`, found `usize`
}
