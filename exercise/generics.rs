fn main() {

    println!("Goodbye world!");

    let mut p = Point::from(1, 2.5);

    p.x = 4;
    println!("coords: {}, {}", p.first(), p.y);

    println!("coords: {}, {}", p.first(), p.y);

    println!("{}",format!("\"{}\"", p.y));
}

struct Point<T, U> { x: T, y: U}

impl<T, U> Point<T, U> {

    fn from(x : T, y : U) -> Self {
        Self{x, y}
    }

    fn first(&self) -> &T {
        &self.x
    }
}