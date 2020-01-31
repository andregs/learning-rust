// TODO https://doc.rust-lang.org/book/ch19-04-advanced-types.html

fn main() {
    type_aliases();
    functions_as_pointers();
    how_to_return_closures();
}

fn type_aliases() {
    type Kilometers = i32; // this is a type alias
    let mut x: i32 = 5;
    let mut y: Kilometers = 5;
    
    println!("x is {} and y is {}", x, y);

    x = y;
    y = x;
    
    println!("x is {} and y is {}", x, y);

    // such aliases are useful when you start to have ugly types like
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    f();

    // aliases can be generic
    type Result<T> = std::result::Result<T, std::io::Error>;
    let x: Result<u32> = Ok(3);
    println!("{}", x.unwrap());

    // there's also the ! type that means 'never'
    // it's the value returned by things like 'continue' and 'panic!'
}

fn functions_as_pointers() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    // we can pass functions as values, not only closures
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 3);

    assert_eq!(answer, 8);
    println!("The answer is {}", answer);

    // functions are represented by the type 'fn'
    // closures are represented by the traits Fn, FnMut, FnOnce
}

fn how_to_return_closures() {
    // you could think it's like this:
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    // but Rust doesn't know the size of the closure at compile-time

    // this will do:
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }

    let f = returns_closure();
    println!("5 + 1 is {}", f(5));
}