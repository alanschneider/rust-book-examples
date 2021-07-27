//////////////////////////////////////////////////
// General notes
//
// - Rust uses snake casing for functions and
//   variables (like_this_example)



fn another_function() {
    println!("another_function has been called.");
}

fn main() {
    another_function();
    
    //////////////////////////////////////////////
    // Shadowing
    //////////////////////////////////////////////
    
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("x is {}", x);

    let x = "outer scope";
    println!("x is {}", x);

    {
        let x = "inner scope";
        println!("x is {}", x);
    }

    println!("x is {}", x);

    //////////////////////////////////////////////
    // Scalar types
    //////////////////////////////////////////////

    // Signed Integers
    
    let _a: i8 = 0;
    let _a: i16 = 0;
    let _a: i32 = 0;
    let _a: i64 = 0;
    let _a: i128 = 0;

    // Unsigned Integers
    
    let _a: u8 = 0;
    let _a: u16 = 0;
    let _a: u32 = 0;
    let _a: u64 = 0;
    let _a: u128 = 0;

    // Integer Literals

    let _a = 98_222;      // Decimal
    let _a = 0xff;        // Hex
    let _a = 0o77;        // Octal
    let _a = 0b1111_0000; // Binary
    let _a = b'A';        // Byte (u8 only)

    // The default integer is i32.

    // Integer Overflow
    //
    // The compiler will err out if it can find overflow errors
    // in code.  The default runtime behavior is to generate an
    // Unrecoverable Error panic.  If compiler with the --release
    // flag, Rust will perform two's complement wrapping, eg. 256
    // will become 0, 257 will become 1, etc.
    //

    // let overflow: i8 = 256;

    // Floating point types
    //
    // When not specified, Rust uses f64 as the
    // default floating point type.
    //

    let _a: f64 = 0.0;
    let _a: f32 = 0.0;

    // Boolean types

    let _t = true;
    let _f: bool = false;

    // Character types
    //
    // The char type is 4 bytes in size and
    // represents a Unicode Scalar Value.

    let _c: char = 'z';
    let _z = 'Z';
    let heart_eyed_cat_emoji = '😻';

    println!("heart eyed cat: {}", heart_eyed_cat_emoji);

    //////////////////////////////////////////////
    // Compound types
    //////////////////////////////////////////////

    // Tuple types
    //
    // - Have a fixed length
    // - Can be different types

    let _tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("_tup is ({}, {}, {})", _tup.0, _tup.1, _tup.2);
    
    // Destructuring

    let (x, y, z) = _tup;

    println!("_tup is ({}, {}, {})", x, y, z);
    println!("_tup is {:?}", _tup);
    println!("_tup is {:#?}", _tup); // Pretty print

    // Array types
    //
    // - Also fixed length
    // - Same type
    // - Allocated on the stack, rather than the heap
    // - If an out of range index is specified at runtime,
    //   a panic will occur and terminate the program

    let _arr           = [1, 2, 3, 4, 5];
    let _arr: [i32; 5] = [1, 2, 3, 4, 5]; // Explicit
    println!("_arr: {:?}", _arr);

    // Create a new array of 5 elements, all initialized
    // with the number 3.

    let _arr = [3; 5];
    println!("_arr: {:?}", _arr);
    
}
