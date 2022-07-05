struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = build_user(
        String::from("nestor@example.com"), 
        String::from("Nestor"));

    println!("username: {}, count: {}", user1.username, user1.sign_in_count);

    let user2 = User {
        email: String::from("another@example.com"),
        sign_in_count: 2,
        ..user1
    };

    println!("email: {}, active: {}", user2.email, user2.active);

    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);
    println!("color: {}", black.0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

}