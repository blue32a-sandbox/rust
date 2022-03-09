struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");

    // error: explicit destructor calls not allowed
    // c.drop();

    // 値がスコープ外に出る前に明示的にドロップする
    drop(c);

    println!("CustomSmartPointer dropped before the end of main.");
}
