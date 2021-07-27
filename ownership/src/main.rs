//////////////////////////////////////////////////
// General notes
//
// - When heap objects fall out of scope, the drop
//   function is called.  The author can put code
//   into this function to return the allocated
//   memory.
// - Rust has a special annotation called the
//   Copy trait. If a type implements this, an
//   older variable is still usable after
//   assignment. Rust will not allow this
//   annotation to be used if the type implements
//   the Drop trait.
// - All integer/boolean/floating point/char types
//   implement Copy.
// - Tuples also implement Copy, but only if the
//   types they contain implement Copy, e.g.
//      (i32, i32)
// - The opposite of referencing by using & is
//   dereferencing using *.
// - Rust prevents dangling references by ensuring
//   data will not go out of scope before its
//   reference
// - String literals are string slices

// some_string goes out of scope at the end of the function and 'drop' is called
//
fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}

// some_string is returned and moves out to the calling function
//
fn gives_ownership() -> String {
    let some_string = String::from("goodbye");
    some_string
}

// This function takes ownership of a_string, then moves a_string out to the
// calling function
//
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// some_integer goes out of scope at the end of the function, but nothing
// special happens
//
fn makes_copy(some_integer: i32) {
    println!("makes_copy: {}", some_integer);
}

// This function borrows s (via reference), instead of owning it.
// s will *not* be dropped at the end of the function.
//
fn calculate_length(s: &String) -> usize {
    s.len()
}

// An example of borrowing by mutable reference.
//
fn change(some_string: &mut String) {
    some_string.push_str(" has been changed");
}

// fn dangle() -> &String {
//     let s = String::from("dangler"):
//     &s
// } // s goes out of scope and is dropped. &s is no longer valid

fn no_dangle() -> String {
    let s = String::from("no dangle");
    s
} // s goes out of scope and is moved to the caller

// Example using string slices.
//
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Improved version that borrows a string slice
// as a parameter, which makes this function
// more general and useful
//
fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s: {}", s);

    takes_ownership(s);
    // println!("{}", s); // This will fail

    let s1 = gives_ownership();
    println!("gives_ownership(): {}", s1);

    let s2 = takes_and_gives_back(s1);
    println!("takes_and_gives_back(s1): {}", s2);

    // Use a reference to s2 when calling this function
    //
    let len = calculate_length(&s2);
    println!("len of s2: {}", len);

    let mut s3 = String::from("My string");
    change(&mut s3);
    println!("change(&s3): {}", s3);

    // NOTE: You can only have one mutable reference to a
    //       particular piece of data in a particular
    //       scope.  This is to prevent data races.
    {
        let mut _s = String::from("hello");
        let _r1 = &mut _s;
        // let _r2 = &mut _s; // this will fail
        // println!("{}{}", _r1, _r2);
    }

    // Multiple immutable references are okay, since readers
    // will not impact other readers.
    //
    // However, you can't have a mutable reference with an
    // immutable reference in the same scope, since the
    // owner of the mutable reference could change data
    // while immutable references are being read...
    // 
    {
        let mut _s = String::from("hello");
        let _r1 = &_s;     // okay
        let _r2 = &_s;     // also okay
        // let _r3 = &mut _s; // NOT okay
        // println!("{}{}{}", _r1, _r2, _r3); // fails here
    }

    let s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // error, since this is a mutable borrow...

    println!("first_word(&s): {}", word);

    let word = first_word_improved(&s[..]);
    println!("first_word_improved(&s[..]): {}", word);
    
    let x = 5;
    makes_copy(x);

    {
        // The following snippet shows _s1 being
        // moved into _s2, which invalidates _s1.
        //
        let _s1 = String::from("hello");
        let _s2 = _s1;

        // println!("{}", _s1); // invalid
    }

    {
        // Clone will allow a deep-copy of s1 to
        // s2.
        //
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{} {}", s1, s2);
    }

    {
        // Stack variables are copied, since it
        // is an inexpensive operation. No need
        // to clone.
        //
        let x = 5;
        let y = x;
        println!("{} {}", x, y);
    }

    // Other types of slices
    //
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
