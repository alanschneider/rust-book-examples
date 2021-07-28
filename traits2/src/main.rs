use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("x > y: {} > {}", self.x, self.y);
        } else if self.x < self.y {
            println!("x < y: {} < {}", self.x, self.y);
        } else {
            println!("x == y: {} == {}", self.x, self.y);
        }
    }
}

fn main() {
    let gt = Pair::new(2, 1);
    let lt = Pair::new(3, 4);
    let eq = Pair::new(5, 5);

    gt.cmp_display();
    lt.cmp_display();
    eq.cmp_display();
}
