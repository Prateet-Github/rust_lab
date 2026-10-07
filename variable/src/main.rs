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

    let is_valid = true;

    println!("{is_valid}");

    let tup = (1, "yes", true);

    println!("{tup:?}");
    println!("{}", tup.0);

    let scores = [1, 30, 40];

    println!("{scores:?}");

    let first_score = scores[1];

    println!("{first_score}");
}
