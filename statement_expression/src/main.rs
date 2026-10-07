fn main() {
    println!("Hello, world!");
    let y = return_it(3);
    println!("{y}");
}

fn return_it(x: i32) -> i32 {
    x
}
