use restaurant; // loading our lib under testing

mod common; // loading our external helper functions

// integration tests are placed inside tests/ directory
// they don't need #[cfg(tests)] and they have access only to the pub API
// binary crates (main.rs) usually are small because they cannot be loaded here, and that means they cannot be tested

#[test]
fn it_example() {
    common::setup();
    restaurant::eat_at_restaurant();
    // restaurant::serve_order(); // we don't have access to private stuff
    assert_eq!(1, 1);
}

// if we need to test private behavior, we can write unit tests in the same file where the function is defined
// in other words, unit tests are defined in src/ and integration tests are defined in tests/ directory