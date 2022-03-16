use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");

        // send関数はパラメータの所有権を持ち、値が移動されると、受信者がその所有権を持つようになる
        tx.send(val).unwrap();

        // error: borrow of moved value: `val`
        // println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
