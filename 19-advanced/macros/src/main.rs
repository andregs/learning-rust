use macros::vec2;

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    let v = vec2!['a', 'b', 'd'];
    println!("Vec from our macro: {:?}", v);

    Pancakes::hello_macro(); // function added via derive macro
}

// see my Cargo.toml
