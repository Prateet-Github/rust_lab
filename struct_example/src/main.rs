fn main() {
    let rectangle1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area: {}", area(&rectangle1));

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("Area: {}", area(&rectangle2));
    println!("rectangle1 is {:#?}", rectangle1); // debug print with # it gets pretty printed

    dbg!(&rectangle2); // debug print with dbg! macro
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
