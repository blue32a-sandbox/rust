fn main() {
    listing_10_20();

    listing_10_23();

    listing_10_24();
}

fn listing_10_20() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn listing_10_23() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn listing_10_24() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");

        // `string2` does not live long enough
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
