//////////////////////////////////////////////////
// General notes
//
// - Rust is an expression-based language
//
//////////////////////////////////////////////////

// Functions
//
// - Must declare the type of each parameter
// - Function bodies are made up of a series of
//   statements optionally ending in an expression
// - Expressions DO NOT end with semicolons
//   - Only statements end with semicolons


fn another_function(x: i32, y: i32) {
    println!("x is {}, y is {}", x, y);
}

fn expression_example() -> i32 {
    let x = 5;
    x + 1
}

fn main() {
    another_function(5, 6);

    let x = expression_example();
    println!("x = {}", x);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("y = {}", y);
}
