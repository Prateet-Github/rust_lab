fn main() {
    let x = 30;
    println!("{x}");

    let x = x + 1; // shadowing
    println!("{x}");

    {
        let x = x + 2;
        println!("{x}");
    }

    println!("{x}");
}
