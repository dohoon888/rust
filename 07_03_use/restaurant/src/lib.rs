// 1. module shortcut (use and mod.)
// mod and use look like c++ namepsace

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

// 2. general use path.
// below code is not general way to use.
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// use crate::front_of_house::hosting::add_to_waitlist;

// pub fn eat_at_restaurant() {
//     add_to_waitlist();
// }

// this is general way to use.
// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// 3. Each module is located within another module, but the example of using the score profile.
// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --생략--
// }

// fn function2() -> io::Result<()> {
//     // --생략--
// }

// 4. Provide a new name with the as keyword
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --생략--
// }

// fn function2() -> IoResult<()> {
//     // --생략--
// }

// 5. Export back to pubuse
// If you import a name with the use keyword,
// that name becomes private in the scope of the new location.
//  Combining pub and use allows the code that calls our code to refer to
//  that name as defined in that scope. This technique is 
// called re-exporting because it makes it possible to import items to the scope while also
//  taking them from somewhere else.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}