// TODO https://doc.rust-lang.org/book/ch19-04-advanced-types.html

fn main() {
    type_aliases();
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
