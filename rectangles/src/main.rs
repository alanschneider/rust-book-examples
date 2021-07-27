//////////////////////////////////////////////////
// General Notes
//
// - {:?} and {:#?} are used for the Debug trait
// - Rust supports automatic referencing/dereferencing
//   so that you don't need to do stuff like
//      (&r1).area()
// 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Example of an associated function defined in a new
// impl block. Associated functions do not take self parameters.
//
// Each struct is allowed to have multiple impl blocks.
//
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area: {}", area(&r1));
    println!("r1: {:?}", r1);

    // Pretty-print
    //
    println!("r1: {:#?}", r1);
    println!("r1.area(): {}", r1.area());

    let r2 = Rectangle {
        width: 10,
        height: 40
    };

    let r3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("r2 in r1? {}", r1.can_hold(&r2));
    println!("r3 in r1? {}", r1.can_hold(&r3));

    let sq = Rectangle::square(3);
    println!("sq: {:?}", sq);
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
