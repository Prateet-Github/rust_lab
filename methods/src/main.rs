#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //&self is immutable ref
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor; // Scale the width by the factor
        self.height *= factor;
    }
}

fn main() {
    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Area: {}", rect.area());

    rect.scale(2);
    println!("New Dimensions: {:?}", rect);
}
