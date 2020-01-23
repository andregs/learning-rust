#![allow(dead_code)]

mod front_of_house {
    pub mod hosting { // "pub" exports to the public API
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*
 * crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
 * 
 * at the root of the crate's module structure we have an implicit module called "crate"
 * the "crate" module is either src/main.rs or src/lib.rs
 * 
 * inner (child) code is private by default, but it can access outer (parent) code.
 */

pub fn eat_at_restaurant() {
    // absolute path to a function
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path (relative to the location of eat_at_restaurant)
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super works like '../' in filesystem
    }

    fn cook_order() {}
}

// if we want, we can also move front_of_house mod definition to src/front_of_house.rs
// and hosting mod definition to src/front_of_house/hosting.rs


// unit tests are defined in the same file under testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_example() {
        // unit tests can test private functions:
        serve_order();
        assert!(2 == 2);
    }

    // we can test private functions from private modules as well,
    // like back_of_house::fix_incorrect_order(), but in this case,
    // we need to create the tests inside that module.
}
