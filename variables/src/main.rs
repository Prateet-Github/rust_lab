fn main() {
    // scalar types
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");

    const PI: f64 = 3.14; // for constant var we need to specify the type i.e. f64 here

    println!("The value of PI is: {PI}");

    let is_student: bool = true; // boolean variable
    println!("Is the person a student? {is_student}");

    let choice: char = 'Y';
    println!("The choice is: {choice}");

    // compound types

    let person = ("Prateet", 21, true); // packing a tuple
    println!("{} {}", person.0, person.1); // accessing tuple elements using index
    let (name, age, is_student) = person; // unpacking
    println!("The name is: {name}, age is: {age}, is student? {is_student}");

    println!("The person is: {person:?}"); // using debug formatting to print the tuple

    let arr = [1, 2, 3, 4, 5];

    println!("{:?}", arr); // accessing array elements using index
}
