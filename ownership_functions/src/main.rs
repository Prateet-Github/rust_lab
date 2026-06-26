fn main() {
    let s = String::from("Hello");
    take_ownership(s);

    let x = 5;
    makes_copy(x);

    // return values and scopes

    let s1 = gives_ownership();
    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);

    println!("{s1} & {s3}")
}

fn take_ownership(some_string: String) {
    println!("{some_string}")
}

fn makes_copy(some_int: i32) {
    println!("{some_int}")
}

// return values and scopes

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
