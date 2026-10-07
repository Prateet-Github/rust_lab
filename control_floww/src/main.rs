fn main() {
    let a = [10, 20, 30, 40, 50];

    // use .iter in array
    for element in a.iter().rev() {
        println!("value: {element}");
    }

    for i in 1..5 {
        println!("{i}");
    }
}
