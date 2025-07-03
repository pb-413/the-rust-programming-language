// 7.2 Grouping Related Code in Modules

mod front_of_house {
    pub mod hosting {                // 7.3 made public
        pub fn add_to_waitlist() {}  // 7.3 made public

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


// 7.3 Exposing Paths with the pub Keyword

// See that the sibling above is private but accessible
// while it's child modules and members of those modules
// (functions) need published / made public.

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


// 7.3 Starting Relative Paths with super

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super signals a suspicion that the relative
        // relationship will be intact in the future.
        super::deliver_order();
    }

    fn cook_order() {}
}


// 7.3 Making Structs and Enums Public

mod back_of_house_struct {
    pub struct Breakfast {
        pub toast: String,
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
}

pub fn eat_at_restaurant_struct() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house_struct::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}

mod back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_enum() {
    let order1 = back_of_house_enum::Appetizer::Soup;
    let order2 = back_of_house_enum::Appetizer::Salad;
}
