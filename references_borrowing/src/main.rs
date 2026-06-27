fn main() {
    let s1 = String::from("hello");
    let len1 = calculate_length(s1);
    // println!("String: {s1}");       // compile error
    println!("{len1}");

    // references & borrowing
    let s2 = String::from("helloo");
    println!("{s2}"); // works fine
    let len2 = calculate_length_r(&s2); //borrowing s2, passing a reference to the function
    println!("{len2}");

    // mutable references
    let mut s4 = String::from("hello");
    change(&mut s4);
    println!("{s4}");

    // dangling references
    // let s5 = dangle(); // compile error

   let s5 = dangle(); // works fine, returns a String instead of a reference
   println!("{s5}");
}

fn calculate_length(s: String) -> usize {
    s.len()
}

// references & borrowing
fn calculate_length_r(s3: &String) -> usize {
    // s3 is a reference to a String
    s3.len()
}

// mutable references
fn change(s: &mut String) {
    s.push_str(", world");
}

// dangling references

fn dangle() -> String {
    let s = String::from("hello");
    // &s // s goes out of scope here, and the reference would be pointing to invalid memory
    s
}
