fn main() {
    println!("Hello, world!");

    // Functions can be called from other functions
    another_function();

    // Functions can take parameters
    show_value(5);

    // statement vs expression

    let y = {
        let x = 3; // This is a statement, it does not return a value
        x + 1 // This is an expression, it returns a value
    };
    println!("The value of y is: {y}");

    // Functions can return values
    let z = five();
    println!("The value of z is: {z}");

    let q = add(2, 3);
    println!("The value of q is: {q}");
}

fn another_function() {
    println!("Another function!");
}

fn show_value(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // This is an expression, it returns a value or do return 5;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
