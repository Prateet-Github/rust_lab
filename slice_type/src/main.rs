fn main() {
    // without slices
    let mut s = String::from("helloo world");
    let word = first_word(&s); // word will get the value 5
    // s.clear(); // uncommenting for slices will cause a compile-time error
    println!("The first word is: {}", word);

    // with slices
    let x = &s[0..2];
    println!("The first word is: {}", x);
}

// without slices
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
