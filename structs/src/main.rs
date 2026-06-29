fn main() {
    //user 1
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

    // user 2
    let user2 = build_user(
        String::from("test@example.com"),
        String::from("someusername123"),
    );

    println!("Generated Username: {}", user2.username);
    println!("Generated Email: {}", user2.email);

    // user 3
    let user3 = User {
        username: String::from("anotheruser"),
        ..user1 // This syntax copies the remaining fields from user1
    };

    println!("User3 Username: {}", user3.username);
    println!("User3 Email: {}", user3.email);
    println!("User3 Sign-in Count: {}", user3.sign_in_count);
    println!("User3 Active: {}", user3.active);

    // user 4

    let user4 = User {
        active: user1.active, // Copying the active field from user1
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("User4 Username: {}", user4.username);
    println!("User4 Email: {}", user4.email);
    println!("User4 Sign-in Count: {}", user4.sign_in_count);
    println!("User4 Active: {}", user4.active);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// fn build_user(email: String, username: String) -> User {
// User {
// active: true,
// username: username,
// email: email,
// sign_in_count: 1,
// }
// }

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
