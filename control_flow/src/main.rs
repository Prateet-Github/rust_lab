fn main() {
    let n = 10;

    if n < 10 {
        println!("n is less than 10");
    } else {
        println!("n is greater than or equal to 10");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // expression evaluates to a value
    println!("number: {number}");
}
