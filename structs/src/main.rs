struct User {
    active:bool,
    username: String,
    email:String,
    sign_in_count:u64,
}

fn build_user(email:String, username:String)-> User {
    User {
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}

fn main() {
    println!("* * * structs * * *");
    {
        let user1 = build_user(String::from("jean@gmail.com"), String::from("jean"));

    }
}
