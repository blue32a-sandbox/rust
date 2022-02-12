mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;

// Re-exporting
pub use crate::front_of_house::hosting;

// which is unidiomatic
// use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// 同じ名前を使用するには、親モジュールを使用するか、新しい名前を付ける
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }
