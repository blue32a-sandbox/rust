// it’s idiomatic
use std::collections::HashMap;

// 外部パッケージの使用
use rand::Rng;

// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io}; // ネストされたパス

// use std::io;
// use std::io::Write;
use std::io::{self, Write}; // 一方が他方のサブパスである場合

// グロブ オペレーター
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{}", secret_number);
}
