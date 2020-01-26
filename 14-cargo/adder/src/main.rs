use add_one;
use add_two;

fn main() {
    let x = 3;
    let y = add_one::add_one(x);
    let z = add_two::add_two(x);
    println!("Hello, world! {0} plus one is {1} and {0} plus two is {2}", x, y, z);
}

// See toml files