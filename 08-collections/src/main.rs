#![allow(unused_variables)]

fn main() {
    // similar to ArrayList in Java
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);

    let v2 = vec![1, 2, 3, 4, 5]; // shorthand

    let third: i32 = v1[2];
    let third2: &i32 = &v1[2];
    println!("The third element is {} {}", third, third2);

    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; // note s1 has been moved here and can no longer be used
    // println!("1 {} 2 {} 3 {} = {}", s1, s2, s3, s); // does not compile

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // nothing moves
    println!("1 {} 2 {} 3 {} = {}", s1, s2, s3, s); // does compile

    let hello = String::from("hello?");
    let maca = String::from("maçã¿");
    println!("hello has {} bytes; maçã has {} bytes.\n", hello.len(), maca.len()); // 5 and 6

    for c in hello.chars() { print!("{} ", c); }
    print!("\n");

    for c in maca.chars() { print!("{} ", c); }
    print!("\n");

    maps();
    exercise1();
    exercise2();
    exercise3();
}

fn maps() {
    use std::collections::HashMap;

    // insert or overwrite a value

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 20);

    for (key, value) in &scores {
        println!("{}: {}", key, value); // blue is 20
    }

    // keep old value or insert a new one
    
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores); // blue is 10

    // read and update (e.g. increment)

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // world is 2
}

fn exercise1() {
    /* Given a list of integers, use a vector and return the mean (the average value), median (when 
     * sorted, the value in the middle position), and mode (the value that occurs most often; a hash
     * map will be helpful here) of the list. */

     let integers = vec![1, 2, 100, 2, 9, 5, 1, 10, 1];
     // avg=14,55555556 mode=1 median=2
     
     let av = average(&integers);
     let me = median(&integers);
     let mo = mode(&integers);
     println!("avg {:.2} med {} mod {}", av, me, mo);
}

fn average(integers: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for i in integers {
        sum += i;
    }

    sum as f32 / integers.len() as f32
}

fn median(integers: &Vec<i32>) -> &str {
    "???" // TODO implement me
}

fn mode(integers: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut modc = 0;
    let mut modv = &integers[0];
    
    for i in integers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
        if *count > modc {
            modc = *count;
            modv = i;
        }
    }

    *modv
}

fn exercise2() {
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word 
    // and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added 
    // to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
    println!("{}", pig_latin("apple"));
    println!("{}", pig_latin("first"));
    println!("{}", pig_latin("Amazônia"));
    println!("{}", pig_latin("Zurique"));
}

fn pig_latin(txt: &str) -> String {
    let mut cseq = txt.chars();
    let first = cseq.next().unwrap();
    let rest = String::from(cseq.as_str());

    let f_low = first.to_ascii_lowercase();

    let result: String;

    // TODO this should not work with a non-ascii first letter
    if f_low == 'a' || f_low == 'e' || f_low == 'i' || f_low == 'o' || f_low == 'u' {
        result = String::from(txt) + "-hay";
    } else {
        result = rest + "-" + first.to_string().as_str() + "ay";
    }

    result
}

fn exercise3() {
    // Using a hash map and vectors, create a text interface to allow a user to add employee names 
    // to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
    // Then let the user retrieve a list of all people in a department or all people in the company by
    //  department, sorted alphabetically.

    // TODO too much work, maybe later
}