struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let example_email = "someone@example.com";
    let example_username = String::from("someusername123");

    let user1 = build_user(example_email.to_string(), example_username);

    println!("{:?}", user1.username);
}
