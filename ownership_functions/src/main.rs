fn main() {
    let s = String::from("Hello");
    take_ownership(s);

    let x = 5;
    makes_copy(x);
}

fn take_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_int: i32) {
    println!("{some_int}")
}
