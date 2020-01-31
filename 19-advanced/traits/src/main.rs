use std::fmt;
use std::ops::Add;
use std::ops::Deref;

fn main() {
    default_type_param_and_op_overload();
    disambiguation();
    disambiguation_on_assoc_functions();
    supertraits();
    newtype_pattern();
}

struct Foo {}

// when you have an existing trait and want to make it a bit more generic,
// you can add an associated type without refactoring everywhere

mod assoc {
    pub trait Iterator {
        type Item;
    
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    impl Iterator for crate::Foo {
        type Item = u32;
    
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }    
    
    // we cannot have multiple implementations with associated types
    // impl Iterator for crate::Foo {
    //     type Item = String;
    // 
    //     fn next(&mut self) -> Option<Self::Item> {
    //         None
    //     }
    // }
}

// when you have an existing trait and want to make it really generic,
// you can add generics in the definition but that means refactoring every usage

mod generic {
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }

    // we can have multiple implementations with generics

    impl Iterator<u32> for crate::Foo {
        fn next(&mut self) -> Option<u32> {
            None
        }
    }
    
    impl Iterator<String> for crate::Foo {
        fn next(&mut self) -> Option<String> {
            None
        }
    }
}

// another option to make a trait more generic without too much refactoring is
// by adding a default generic type parameter

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point { // no type param here, we're using default
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn default_type_param_and_op_overload() {
    let a = Point { x: -2, y: 3 };
    let b = Point { x: 1, y: 2 };
    let c = Point { x: -1, y: 5 };

    assert_eq!(a + b, c); // yeap, we are overloading the operator + 

    let a = Milimeters(42);
    let b = Meters(2);
    let c = Milimeters(2042);

    assert_eq!(a + b, c);
}

// this compiles OK, but note we are 'use std::ops::Add' at line #1
// trait Add<RHS=Self> { // specifying default type param value here
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

#[derive(Debug, PartialEq)]
struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters { // we are not using default here
    type Output = Milimeters;

    fn add(self, other: Meters) -> Milimeters {
        Milimeters(self.0 + other.0 * 1000)
    }
}

fn disambiguation() {
    trait Pilot {
        fn fly(&self);
    }
    
    trait Wizard {
        fn fly(&self);
    }
    
    struct Human; // curly brackets not needed
    
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // it's a little bit similar to multiple inheritance in oop

    let person = Human;
    Pilot::fly(&person); // captain speaks
    Wizard::fly(&person); // up
    person.fly(); // waving arms (this is the default)
    Human::fly(&person); // waving arms
}

fn disambiguation_on_assoc_functions() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    // to specify what fn we want to call, we can use the fully qualified name

    println!("A baby dog is called {}", Dog::baby_name());
    // println!("A baby dog is called {}", Animal::baby_name()); // does not compile
    println!("A baby dog is called {}", <Dog as Animal>::baby_name());
}

fn supertraits() {
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string(); // method found in self because we 'inherit' from Display
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {} // only compiles if Point implements Display

    impl fmt::Display for Point { // this makes the previous line compile
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let p = Point { x: 25, y: 0 };
    p.outline_print();
}

fn newtype_pattern() {
    // sometimes we want to implement an external trait for an external type, but that's impossible.
    // a workaround would be wrapping the type in a local type. let's implement Display for Vec<String>:

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "vec [{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("output: {}", w);

    // however, Wrapper is not a Vec. If we want Wrapper to implement all methods from Vec, we can Deref

    impl Deref for Wrapper {
        type Target = Vec<String>;
        fn deref(&self) -> &Vec<String> {
            &self.0
        }
    }

    // only compiles because we implemented Deref to extract the vector out of the Wrapper tuple
    println!("our vector size is {}", w.len());

    // now we can say Vec<String> implements Display
}
