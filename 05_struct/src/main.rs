struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

fn build_user(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        username: String::from("name1"),
        email: String::from("user_aaa@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1:{}, {}", user1.username, user1.sign_in_count);

    let mut user2 = User {
        username: String::from("name2"),
        email: String::from("user_aaa@example.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("user2:{}", user2.active);
    user2.active = false;
    println!("user2:{}", user2.active);

    let user3 = build_user("name3", "email3");
    println!("user3:{}, {}", user3.username, user3.email);

}
