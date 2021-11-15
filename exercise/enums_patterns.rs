fn main() {

    let tuple : (u8, u8, String) = (1, 1, String::from("Red"));

    print!("{:?}", tuple);

    let black : Color = Color(0.0, 0.0, 0.0);
    println!("{:?}", black);

    let re = Rect::new(4.3, 2.0);

    println!("{}", re.area());

}

#[derive(Debug)]
struct Color(f32, f32, f32);

#[derive(Debug)]
struct Rect {
    w: f32,
    h: f32,
}

impl Rect {
    pub fn new(w : f32, h : f32) -> Self {
        Self {w, h}
    }

    pub fn area(self) -> f32 {
        self.w * self.h
    }
}