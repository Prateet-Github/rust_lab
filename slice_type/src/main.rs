fn main() {
    // without slices
    let mut s = String::from("helloo world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // uncommenting for slices will cause a compile-time error
    println!("The first word is: {}", word);

    // with slices
    // let x = &s[0..2];
    // println!("The first word is: {}", x);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("The slice is: {:?}", slice);
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
