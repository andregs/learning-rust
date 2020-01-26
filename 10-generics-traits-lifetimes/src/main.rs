
fn main() {
    let number_list = vec![12, 85, 43, 77, 99, 11];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'P', 'c', 'Y', 'x'];

    let result = largest(&char_list);
    println!("The largest character is {}", result);

    // different concrete implementations of a generic can have different funtions
    // note sum() and concat() in the same Point<T>

    let int_point = Point { x: 3, y: 2 };
    println!("sum {:?} = {}", int_point, int_point.sum());
    
    let str_point = Point { x: "foo", y: "bar" };
    println!("concat {:?} = {}", str_point, str_point.concat());

    let x = String::from("xyz");
    let y = "abcd";
    let r = longest(x.as_str(), y);
    println!("The longest is {}", r);

    // this is equivalent to the previous block, but it does not compile
    // because y and r have the same lifetime inside 'longest', 
    // while here y dies earlier than r:
    // let x = String::from("xyz");
    // let r;
    // {
    //     let y = String::from("abcd");
    //     r = longest(x.as_str(), y.as_str());
    // }
    // println!("The longest is {}", r);
}

// takes a generic slice and return the largest item
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest { // only generics that implement PartialOrd trait are sortable
            largest = item;
        }
    }

    largest
}

#[derive(Debug)] // allow println
struct Point<T> {
    x: T,
    y: T,
}

impl Point<i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}

impl Point<&str> {
    fn concat(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

// a "blanket implementation" let us implement a trait for any type that implements another trait
// e.g. std implements the ToString trait on any type that implements the Display trait:
// impl<T: Display> ToString for T {
//     // --snip--
// }

// lifetimes: this does not compile, Rust doesn't know the lifetimes
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// when Rust cannot infer the lifetimes, you have to provide them like generics.
// it's just a compilation help, it does not actually change the lifetime of the references.
// in this example, we are saying all args and returned reference have the same lifetime, 
// that means they are equivalent to the one with the shorter lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
