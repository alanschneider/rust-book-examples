//////////////////////////////////////////////////
// General Notes
//
// - Lifetime Annotation Syntax:
//   - Don't change how long refs live
//   - Functions can accept refs with any lifetime
//     by specifying a generic lifetime parameter
//   - Describes the relationships of multiple ref
//     lifetimes without affecting the lifetime
//   - Lifetime parameters start with single apostrophe (')
//     - By convention, most people use 'a
//
//   Examples:
//    &i32         // a reference
//    &'a i32      // a reference with an explicit lifetime
//    &'a mut i32  // a mutable reference with an explicit lifetime
//
// - The Rust compiler has lifetime elision rules that automatically infer
//   the lifetimes of certain references so that they do not need to be
//   specified in all cases.
//
//    1. Each param gets its own lifetime param, e.g.
//
//          fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {...}
//
//    2. If there is only one input lifetime param, that lifetime
//       is assigned to the output param, e.g.
//
//          fn foo<'a>(x: &'a i32) -> &'a i32 {...}
//
//    3. If there are multiple lifetime input params, but one of
//       them is &self or &mut self, the lifetime of self is
//       assigned to all output lifetime parameters, e.g.
//
//    See section 10.3 of the Rust book for good examples of lifetime
//    ambiguity and why the compiler would need it explicitly
//    defined in certain cases...

// In this example, the parameters and return value must
// have the same lifetime. x must live at least as long as y, and
// vice-versa. The borrow checker will reject anything that
// doesn't adhere to these constraints.
//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Example of a function that elides the lifetime parameters,
// since they can be infered from the code by the compiler.
//
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ImportantExcerpt cannot outlive its reference to
// part...
//
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 1st and 3rd rules apply.
    //
    // Each param gets its own lifetime parameter, and since
    // the first param is self, the output parameter will have
    // the same lifetime as self.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

use std::fmt::{Display, Formatter, Result};
impl Display for ImportantExcerpt<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "------> {} <------", self.part)
    }
}

fn announce_the_longest_string<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Attention: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("abcd");
    let s2 = "xyz";
    let r = longest(s1.as_str(), s2);

    println!("longest: {}", r);

    // An example of references with different lifetimes
    //
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // An example of code that would fail
    //
    /*
       let string1 = String::from("long string is long");
       let result;
       {
           // Since the longest function could return a ref
           // to either string1 or string2, this fails because
           // string2 will be dropped once this falls out of
           // scope, therefore invalidating result.
           //
           let string2 = String::from("xyz");
           result = longest(string1.as_str(), string2.as_str());
       }
       println!("The longest string is {}", result);
    */

    let moby_dick = String::from("Call me Ishmael. Some years ago...");
    let s1 = moby_dick.split('.').next().expect("No '.' found...");

    // The excerpt will live as along as s1 is still valid
    //
    let excerpt = ImportantExcerpt { part: s1 };

    println!("excerpt.part: {}", excerpt.part);
    announce_the_longest_string(s1, s2, excerpt);
    
    let s: &'static str = "I will live on until the program stops";
    println!("Static lifetime string: {}", s);
}
