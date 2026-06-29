fn main() {
    let user1 = User {
        username: String::from("prateettiwari"),
        email: String::from("prateettiwari29@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Sign-in Count: {}", user1.sign_in_count);
    println!("Active: {}", user1.active);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
