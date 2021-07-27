//////////////////////////////////////////////////
// General Notes
//
// - enums used for enumerating all possible variants
// - Can have data associated with it

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

/*
struct IpAddr {
    kind: IpAddrKind,
    address: String
}
 */

// Better example using an enum variant
//
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("self.call(): {:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    /*
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
     */

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let m = Message::Quit;
    m.call();

    let m = Message::Move { x: 10, y: 20 };
    m.call();

    let m = Message::ChangeColor(0, 10, 20);
    m.call();

    matching_enums();
    options();
}

fn route(ip_kind: IpAddrKind) {
    println!("route: {:?}", ip_kind);
}

#[derive(Debug)]
enum UsState {
    Alabama, Alaska, California, Colorado, Delaware, Georgia, Idaho, Kentucky
}

#[derive(Debug)]
enum Coin {
    Penny, Nickel, Dime, Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("LUCKY PENNY");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn matching_enums() {
    println!("A penny is worth {}", value_in_cents(Coin::Penny));
    println!("A nickel is worth {}", value_in_cents(Coin::Nickel));
    println!("A dime is worth {}", value_in_cents(Coin::Dime));
    println!("A quarter is worth {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}

fn options() {
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_val = 0u8; // suffixed literal
    match some_u8_val {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    };

    // Use if let syntax if you don't want to cover
    // all possible matches
    //
    // if let can also be used with else
    //
    let some_u8_val = Some(0u8);
    if let Some(3) = some_u8_val {
        println!("three");
    }
    else {
        println!("not three");
    }
    
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
