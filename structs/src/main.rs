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

fn to_string_user(user: &User) -> String {
    format!(
        "active:{} username:{} email:{} sign_in_count:{}",
        &user.active, &user.username, &user.email, &user.sign_in_count
    )
}

fn main() {
    println!("* * * structs * * *");
    {
        let user1 = build_user(String::from("jean@gmail.com"), String::from("jean"));

        let user2 = User {
            username: user1.username.clone(),
            email: String::from("paul@gmail.com"),
            ..user1
        };
        println!("user1:{}", to_string_user(&user1));

        println!("user2:{}", to_string_user(&user2));
    }
}
