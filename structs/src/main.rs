use std::fmt::Debug;

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
//Tuple Struct
struct Color(i32, i32, i32);
fn to_string_color(color: &Color) -> String{
    format!("{{r:{}, g:{}, b:{}}}", color.0, color.1, color.2) // {{}} will recognize `{}` as a string
}
struct Point(i32, i32, i32);
fn to_string_point(point: &Point) -> String {
    format!("{{x:{}, y:{}, z:{}}}", point.0, point.1, point.2)
}

//Example Struct Usage
#[derive(Debug)] // trait for logging struct 
struct Rectangle {
    width:u32,
    height:u32,
}
// manually implement fmt method
// impl Debug for Rectangle {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Rectangle").field("width", &self.width).field("height", &self.height).finish()
//     }
// }


//Unit-Like Struct without Any Field
struct AlwaysEqual;
fn main() {
    println!("* * * structs * * *");
    {
        let user1 = build_user(String::from("jean@gmail.com"), String::from("jean"));

        let user2 = User {
            // username: user1.username.clone(),
            username: user1.username.to_owned(),
            email: String::from("paul@gmail.com"),
            ..user1
        };
        println!("user1:{}", to_string_user(&user1));
        println!("user2:{}", to_string_user(&user2));
    }

    println!("\n* * * Tuple Struct * * *");
    {
        let black = Color(0,0,0);
        let origin = Point(0,0,0);

        println!("black:{}", to_string_color(&black));
        println!("origin:{}", to_string_point(&origin));
    }

    println!("* * * Unit-Like Struct * * *");
    {
        let _subject = AlwaysEqual;
    }

    println!("* * * Example Struct Usage * * *");
    {
        let scale = 2;
        let rect1 = Rectangle {
            width:dbg!(30 * scale),
            height:50
        };
        println!("Area: {}", area(&rect1)); // 1500
        // println!("{:?}", rect1); // one line
        // println!("{:#?}", rect1); // applicate prettier format
        dbg!(&rect1); //macro that use instance's ref to debug

        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
    }
}
