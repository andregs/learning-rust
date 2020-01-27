use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{{Rc, Weak}};

use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};

fn main() {
    
    // let's try to create two owners for one value, like this graph:
    // b -> 6 \
    //    a -> 1 -> 2 -> 3 -> Nil
    // c -> 9 / 
    
    let _a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let _b = Cons(6, Box::new(_a));
    // let _c = Cons(9, Box::new(_a)); // does not compile because _a moved into _b, _b now owns _a

    // we can have multiple owners by using Rc (ref-count). It's like the 1st person turning a TV on in a room,
    // others can join the room to watch TV, but the last person to quit turns the tv off.
    let _a = Rc::new(RcCons(1, Rc::new(RcCons(2, Rc::new(RcCons(3, Rc::new(RcNil)))))));
    let _b = RcCons(6, Rc::clone(&_a));
    let _c = RcCons(9, Rc::clone(&_a));

    // same thing, but with some println so we can see the counter
    let _a = Rc::new(RcCons(1, Rc::new(RcCons(2, Rc::new(RcCons(3, Rc::new(RcNil)))))));
    println!("count after creating a: {}", Rc::strong_count(&_a));
    let _b = RcCons(6, Rc::clone(&_a));
    println!("count after creating b: {}", Rc::strong_count(&_a));
    {
        let _c = RcCons(9, Rc::clone(&_a));
        println!("count after creating c: {}", Rc::strong_count(&_a));
    }
    println!("count after c dies: {}", Rc::strong_count(&_a));
    // the whole graph dies when the three owners are out of scope (when ref-count reaches 0)

    hello("Rust");

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Rust does automatic deref here &MyBox<String> into &String into &str
    hello(&(*m)[..]); // if we didn't have automatic deref

    let _c = CustomSmartPointer { data: String::from("my stuff") };
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created. They will live untill the end of main.");

    let c = CustomSmartPointer { data: String::from("dies soon") };
    println!("CustomSmartPointer created but it will die.");
    drop(c); // we can drop early when we need, as example, to release a lock
    println!("End of main scope.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// does not compile
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> { // there's also DerefMut to deref mutable references
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,

    // Rc because Node owns its children and we want to share the ownership with vars so we can access each Node directly
    // RefCell because we want to modify which nodes are children of another node
    children: RefCell<Vec<Rc<Node>>>,

    // Weak is like Rc without ownership.
    // when parent is dropped we want children to be dropped as well (parent owns children).
    // when child is dropped we don't want parent to be dropped.
    // if we change Weak by Rc here, we would create a cycle between parent <-> child
    // the cycle would result in a memory leak, because ref-count would never reach 0.
    parent: RefCell<Weak<Node>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let x = 5;
        let y = &x;

        assert_eq!(x, 5);
        assert_eq!(*y, 5); // deref let us follow a pointer
    }

    #[test]
    fn deref_box() {
        let x = 5;
        let y = Box::new(x); // similar to &x

        assert_eq!(x, 5);
        assert_eq!(*y, 5); // deref a Box
    }

    #[test]
    fn deref_my_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(x, 5);
        assert_eq!(*y, 5); // works because impl Deref for MyBox
        // "*y" behind the scenes is "*(y.deref())"
    }

    #[test]
    fn test_tree() {
        // 5 -s-> 3
        // 5 <-w- 3

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        // run with "cargo test -- --nocapture" to show stdout content
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    }
}
