fn main() {
    some_examples();
    while_let_loops();
    for_loops();
    fn_params();
    match_multiple();
    match_range();
    match_char_range();
    destructuring();
}

fn some_examples() {
    println!("\nSome examples of pattern matching");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        // 'color' var extracted from the Option
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 { // age is extracted from Result and shadows the original var
            println!("Using purple as the brackground color");
        } else {
            println!("Using orange as the brackground color");
        }
    } else {
        println!("Using blue as the brackground color");
    }
}

fn while_let_loops() {
    println!("\nLooping while pattern continues to match");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("Here's the final vector {:?}", stack); // empty, it was consumed
}

fn for_loops() {
    println!("\nIterate and destruct");
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn fn_params() {
    println!("\nDestructing params");
 
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}

fn match_multiple() {
    println!("\nMatch expression with multiple patterns");

    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"), // comment this line and match no longer compiles
    }
}

fn match_range() {
    println!("\nMatch expression with range patterns");
    
    let x = 5;
    match x {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }
}

fn match_char_range() {
    println!("\nMatch expression with range of chars");
    
    let x = 'e';
    match x {
        'a'..='m' => println!("Early ASCII letter"),
        'n'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }
}

fn destructuring() {
    println!("\nDestructuring...");

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // destruct & alias

    assert_eq!(a, 0);
    assert_eq!(b, 7);

    let Point { x, y } = p; // destruct - same name
    
    assert_eq!(x, 0);
    assert_eq!(y, 7);

    match p {
        Point { x, y: 0 } => println!("On the X axis at {}", x),
        Point { x: 0, y } => println!("On the Y axis at {}", y),
        Point { x, y } => println!("On neither axis ({}, {})", x, y),
    }

    #[derive(Debug)]
    struct ManyFields {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    let many = ManyFields { a: 0, b: 1, c: 2, d: 3 };
    match many {
        // use .. to ignore the rest of the pattern
        ManyFields { a, .. } => println!("a is {} in {:?}", a, many),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // note the guard
        Some(x) => println!("{}", x),
        None => (), // swallow
    }

    enum Message {
        Hello { id: i32 },
    }
    
    let msg = Message::Hello { id: 5 };
    
    match msg {
        // note the syntax to test a pattern and store the matching value in a variable
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable);
        },
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        },
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        },
    }
}
