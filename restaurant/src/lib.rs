//////////////////////////////////////////////////
// General Notes
//
// The Module System:
//
//    Packages:        A Cargo feature that lets you build, test, and share crates
//    Crates:          A tree of modules that produces a library or executable
//    Modules and use: Let you control the organization, scope, and privacy of paths
//    Paths:           A way of naming an item, such as a struct, function, or module
//

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // This public associated function is needed
        // because the user is unable to instantiate
        // Breakfast, due to seasonal_fruit being
        // private
        //
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // When an enum is defined pub, all its members
    // are pub
    //
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// Bringing a module into scope idiomatically
//
//use crate::front_of_house::hosting;

// Bringing a module into scope using a relative path
//
//use self::front_of_house::hosting;

// Bringing a function into scope in an unidiomatic way. This
// is discouraged, since it isn't clear that the function
// isn't locally defined.
//
//use crate::front_of_house::hosting::add_to_waitlist;

// Re-exporting. This allows external code to have access
// to hosting. It also simplifies the structure; external
// clients may not be interested in front_of_house/back_of_house.
// 
// Re-exporting is useful when the internal structure of
// the code is different from how programmers calling your
// code would think about the domain...
//
pub use crate::front_of_house::hosting;

// However, when bringing in structs, enums, and other items,
// it's idiomatic to specify the full path
//
use std::collections::HashMap;

// The exception to the above is bringing in two items with
// the same name but different scope
//

/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
}

fn function2() -> io::Result<()> {
}
 */

// Another way to resolve the above example is to rename
// the item
//

/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
}

fn function2() -> IoResult<()> {
}
 */

// Using nested paths to clean up large use lists
//
// Before:
//
// use std::cmp::Ordering;
// use std::io;
//
// After:
//
// use std::{cmp::Ordering, io}

// Before:
//
// use std::io;
// use std::io::Write;
//
// After
//
// use std::io::{self, Write};

// Using a glob operator:
//
use std::collections::*;


pub fn eat_at_restaurant() {
    // Absolute path
    //

    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //
    
    //front_of_house::hosting::add_to_waitlist();

    // use statement above brings hosting into scope...
    //

    hosting::add_to_waitlist();

    // In the example of
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("meal.toast: {}", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries"); // this will fail

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
