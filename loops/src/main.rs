fn main() {
    // loop{
    //     println!("again!");
    // }

    // returning values from loops
    // let mut x = 0;

    // let result = loop {
    //     x = x + 1;
    //     if x == 10 {
    //         break x; // always  returns a value, in this case the value of x
    //     }
    // };

    // println!("The result is {result}");

    // Disambiguating with Loop Labels

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Looping through a collection with while

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;
    while i < 5 {
        println!("the value is: {}", a[i]);
        i += 1;
    }

    // Looping through a collection with for is more efficient and less error-prone than using a while loop. The for loop takes care of the logic of iterating through the collection, so you don’t have to create a variable and manually update it.

    for i in a {
        println!("the value is: {i}");
    }

    // Range syntax with for loops

    for number in 1..=4 {
        println!("{number}");
    }

    // rev method to reverse the range
    for number in (1..=4).rev() {
        println!("{number}");
    }
}
