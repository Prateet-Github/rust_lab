fn main() {
    // variable scope
    let s = "Hello, world!";

    {
        let x = "This is a block scope.";
        println!("{x}");
    }

    println!("{s}");
    // println!("{x}");

    // the string type
    let mut s1 = String::from("Hello, heap");
    s1.push_str(" & string");
    println!("{s1}");

    // memory and allocation
    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid

    // variable and data interacting with move

    let mut p = 3;
    let q = p; // copy
    p = 10;
    println!("p: {p}, q: {q}");

    let s2 = String::from("hello");
    let s3 = s2; // move (not shallow copy)
    // println!("{s2}"); // this would cause a compile error
    println!("{s3}");
}
