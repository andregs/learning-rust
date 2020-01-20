#![allow(unreachable_code, dead_code, unused_variables)]

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> means all errors
    // error_handling_with_match();
    // error_handling_with_closures();
    error_handling_with_expect();
    
    // by default, main returns "()"", but we can return a Result to propagate errors
    propagate()?;
    Ok(())
}

fn error_handling_with_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => {
            println!("File open - OK");
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File creation - OK");
                    fc
                },
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn error_handling_with_closures() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn error_handling_with_expect() {
    let f = File::open("hello.txt").unwrap_or(
        File::create("hello.txt").expect("Problem creating the file.")
    );
}

fn propagate() -> Result<String, io::Error> {
    // read_username_from_file_long_way()?;
    // read_username_from_file_short_way()?;
    // read_username_from_file_even_shorter_way()?;
    read_username_from_file_shortest_way()
}

fn read_username_from_file_long_way() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // we don't need 'return' here because it's the last expression
    }
}

// same as read_username_from_file_long_way but shorter, thanks to '?' operator
fn read_username_from_file_short_way() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // there is one difference between '?' operator and 'match' in the previous fn:
    // '?' operator calls the 'from' function defined in 'From' trait to convert errors
}

// again, same thing
fn read_username_from_file_even_shorter_way() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// last time, I promise
fn read_username_from_file_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
