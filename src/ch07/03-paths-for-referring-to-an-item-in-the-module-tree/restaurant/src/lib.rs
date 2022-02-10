mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    // private module
    mod serving {
        // private function
        fn take_order() {}
    }
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,

        // private field
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();


    // 構造体と列挙型を公開する

    // 夏にライ麦パンで朝食を注文する
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // どんなパンが欲しいか考えを変える
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");

    // 列挙型をパブリックとして指定すると、そのすべてのバリアントがパブリックになる
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


