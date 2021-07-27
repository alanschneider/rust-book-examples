//////////////////////////////////////////////////
// General Notes
//
// - RUST_BACKTRACE=1 cargo run
//

fn main() {
    panic_examples();
    recoverable_errors_examples();
    unwrap_example();
    expect_example();
    error_propagation_example();
}

// Use panic! for unrecoverable errors that should
// stop the application
//
fn panic_examples() {
    // panic!("crash and burn");

    let _v = vec![1, 2, 3];
    // _v[99]; // uncomment to panic
}

use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn recoverable_errors_examples() {
    let file_name = "hello.txt";
    let file_result = File::open(&file_name);

    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_name) {
                Ok(new_file) => new_file,
                Err(create_error) => panic!("Count not create {}: {:#?}", &file_name, create_error),
            },
            other_error => panic!("Could not open {}: {:#?}", file_name, other_error),
        },
    };
}

// Unwrap is a shortcut method that returns the value
// stored in the Result if it is Ok. Otherwise, if the
// Result is Err, it will panic.
//
// Rename the file_name below to see an example error.
//
fn unwrap_example() {
    let file_name = "hello.txt";
    let _f = File::open(&file_name).unwrap();
}

// expect allows the customization of the panic
// message. Rename the file_name to see a failure.
//
fn expect_example() {
    let file_name = "hello.txt";
    let _f = File::open(&file_name).expect(format!("Failed to open {:#?}", &file_name).as_str());
}

fn error_propagation_example() {
    match read_file("hello.txt") {
        Ok(s) => println!("hello.txt: {}", s),
        Err(e) => println!("Failed to read hello.txt:\n{:#?}", e),
    };
    match read_file("helo.txt") {
        Ok(s) => println!("helo.txt: {}", s),
        Err(e) => println!("Failed to read helo.txt:\n{:#?}", e),
    };
}

// Original read_file function
//
/*
fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file_result = File::open(&file_name);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}
*/

// A cleaner version of the above read_file function, using
// the ? operator to automatically propagate errors.
//
// ? can be used anywhere that returns Result, Option, or
// types that implement std::ops::Try.
//
/*
fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(&file_name)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}
 */

// Refactored even more. Chaining the calls.
//
/*
fn read_file(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut file = File::open(&file_name)?.read_to_string(&mut s)?;
    Ok(s)
}
 */

// Reading a file into a string is so common, Rust already provides
// a function to do so.
//
fn read_file(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(&file_name)
}
