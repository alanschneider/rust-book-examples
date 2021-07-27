fn main() {
    vectors();
    strings();
    hash_maps();
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vectors() {
    let v: Vec<i32> = Vec::new(); // empty vector
    println!("v: {:?}", v);

    let v = vec![1, 2, 3]; // using the macro
    println!("v: {:?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v: {:?}", v);

    let third: &i32 = &v[2];
    println!("third element of v: {}", third);

    match v.get(2) {
        Some(x) => println!("x: {}", x),
        None => println!("nothing found at v.get(2)"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        // Need to dereference i to get to its value
        //
        *i += 50;
    }

    for i in &v {
        println!("{}", i);
    }

    // This will panic
    // let does_not_exist = &v[100];

    // This will return None instead
    let does_not_exist = v.get(100);
    println!("does_not_exist: {:?}", does_not_exist);

    // The following code is invalid, since there is a mix
    // of mutable and immutable references. Since vectors
    // can be resized and possibly reallocated, having an
    // immutable reference before the vector is changed
    // could mean that the previous reference points to
    // deallocated memory. The borrow checker will find
    // issues like this upon compilation.
    //
    /*
       {
           let mut v = vec![1, 2, 3];
           let first = &v[0];                           // immutable borrow occurs here
           v.push(6);                                   // mutable borrow later used here
           println!("The first element is: {}", first); // immutable borrow later used here, and is invalid
       }
    */

    // Vectors can only store values of the same type. However,
    // an enum can be used to wrap underlying types
    //
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row: {:?}", row);
}

// - Strings and string slices are UTF-8 encoded
// - Rust does not support indexing into strings
// - String slices could result in runtime panics
//   - Some UTF-8 characters are multi-byte
// - Best approach is to iterate over chars()
//
fn strings() {
    let mut s = String::new(); // new, blank string

    // Ways to create a string from a string literal
    //
    let data = "initial contents";
    let s = data.to_string();
    println!("s: {}", s);

    let s = "other contents".to_string();
    println!("s: {}", s);

    // Strings can grow
    //
    let mut s = "my content".to_string();
    s.push_str(" appended with more content");
    println!("s: {}", s);

    s.push('!');
    println!("s: {}", s);

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s = s1 + &s2; // NOTE: s1 has been moved here and is now invalid

    //
    // The above implements
    //    add(self, s: &str) -> String
    //

    println!("s: {}", s);

    // Using the format macro to concatenate strings
    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s: {}", s);

    // Iterate over each scalar unicode character
    //
    for c in s.chars() {
        println!("c: {}", c)
    }

    // Iterate over bytes
    //
    for b in s.bytes() {
        println!("b: {}", b)
    }
}

use std::collections::HashMap;

// - For types that implement the Copy trait, values are copied into the HashMap
// - For owned values like String, the values will be moved and the HashMap
//   will be the owner of those values
// - Uses SipHash by default
//
fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("scores: {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("scores: {:?}", scores);

    let score = scores.get("Blue");
    println!("score for Blue: {:?}", score);
    match score {
        Some(x) => println!("score for Blue: {}", x),
        None => println!("no score found for Blue")
    }

    scores.insert(String::from("Blue"), 0);
    if let Some(score) = scores.get("Blue") {
        println!("updated score for Blue: {}", score);
    }


    scores.entry(String::from("Red")).or_insert(100); // added
    scores.entry(String::from("Blue")).or_insert(90); // unchanged
    println!("updated scores: {:?}", scores);
    
    for (k, v) in &scores {
        println!("key: {}, value: {}", k, v);
    }

    let n = String::from("fav color");
    let v = String::from("blue");
    let mut map = HashMap::new();
    map.insert(n, v); // n, v have been moved and are no longer valid
    println!("map: {:?}", map);

    let text = "hello world wonderful world";
    let mut word_count_map = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word count: {:?}", word_count_map);
    
}
