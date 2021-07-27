use std::cmp::PartialOrd;

struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> MixedPoint<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &i in list {
        if i > largest {
            largest = i;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("largest: {}", largest(&number_list));

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    println!("largest: {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("largest: {}", largest(&char_list));


    
    let both_ints = Point { x: 5, y: 10 };
    println!("both_ints: {}, {}", both_ints.x(), both_ints.y());
    //println!("distance_from_origin: {}", both_ints.distance_from_origin());

    let both_floats = Point { x: 1.0, y: 4.0 };
    println!("both_floats: {}, {}", both_floats.x(), both_floats.y());
    println!(
        "distance_from_origin: {}",
        both_floats.distance_from_origin()
    );

    let int_and_float = MixedPoint { x: 5, y: 4.0 };
    println!(
        "int_and_float: {}, {}",
        int_and_float.x(),
        int_and_float.y()
    );

    let mp1 = MixedPoint { x: 5, y: 10.4 };
    let mp2 = MixedPoint { x: "hello", y: 'c' };
    let mp3 = mp1.mixup(mp2);
    println!("mp3.x = {}, mp3.y = {}", mp3.x, mp3.y);
}
