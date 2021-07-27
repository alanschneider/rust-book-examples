fn main() {
    let number = 3;

    if number < 5 {
        println!("true");
    }
    else {
        println!("false");
    }

    let number = 7;

    if number < 5 {
        println!("true");
    }
    else {
        println!("false");
    }

    let number = 3;

    if number == 3 {
        println!("number is three");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is NOT divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number);


    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("Liftoff");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("a[{}] = {}", index, a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("val = {}", element);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("liftoff");
}
