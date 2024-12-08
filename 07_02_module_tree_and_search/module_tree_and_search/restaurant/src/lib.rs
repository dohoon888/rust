// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

// 1. Exposing a route with the pub keyword
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // 절대 경로
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 상대 경로
//     front_of_house::hosting::add_to_waitlist();
// }

// 2. Relative path starting with super
// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// 3. Revealing structures, enumeration types
// toast is public. but fruit is not public.
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
 
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
//    // meal.seasonal_fruit = String::from("blueberries");
// }

// 4. public enumeration Type and Variant 
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}