fn main() {
    println!("Hello, world!");
    another_function();
    show_value(5);

    // statement vs expression

    let y = {
        let x = 3; // This is a statement, it does not return a value
        x + 1 // This is an expression, it returns a value
    };
    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function!");
}

fn show_value(x: i32) {
    println!("The value of x is: {x}");
}
