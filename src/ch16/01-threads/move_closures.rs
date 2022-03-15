use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // vの所有権はスレッドに移動したので、メインスレッドではもうvを使用できない
    // drop(v); // oh no!

    handle.join().unwrap();
}
