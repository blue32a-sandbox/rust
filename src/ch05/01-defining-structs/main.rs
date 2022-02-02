struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!(
        "email: {}, username: {}, active: {}, sign_in_count: {}",
        user1.email,
        user1.username,
        user1.active,
        user1.sign_in_count
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!(
        "email: {}, username: {}, active: {}, sign_in_count: {}",
        user2.email,
        user2.username,
        user2.active,
        user2.sign_in_count
    );
}
