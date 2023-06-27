#[derive(Debug)]
enum IpAddrKind {
    //we can put different types and also different struct!
    //That's why this enum is powerful
    V4(u8, u8, u8, u8),
    V6(String),
}
// struct IpAddr {
//     kind:IpAddrKind,
//     address:String,
// }
fn route(_ip_kind: IpAddrKind) {
    // println!("{:#?}", ip_kind);
    // dbg!(ip_kind);
}

#[derive(Debug)]
struct Move {
    x:i32,
    y:i32,
}
#[derive(Debug)]
enum Message {
    Quit,
    Move(Move),
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Call function called");
        dbg!(&self);
    }
}
fn main() {
    println!("* * * Enums * * *");
    println!("\n\n[Definition]");
    {
        println!("\n<Enum Values>");
        {
            //with struct
            // let home = IpAddr {
            //     kind:IpAddrKind::V4,
            //     address:String::from("127.0.0.1"),
            // };
            // let loopback = IpAddr {
            //     kind:IpAddrKind::V6,
            //     address:String::from("::1"),
            // };

            //with enum
            // let home = IpAddrKind::V4(127, 0, 0, 1);
            // let loopback = IpAddrKind::V6(String::from("::1"));
            // route(home);
            // route(loopback);

            let hi_enum = String::from("hi enum");

            let message1 = Message::Move(Move {x:4, y:24});
            let message2 = Message::Quit;
            let message3 = Message::ChangeColor(255, 255, 255);
            let message4 = Message::Write(hi_enum);
            message1.call();
            message2.call();
            message3.call();
            message4.call();
        }
    }
    println!("\n\n[Pattern matching]");
    {

    }
    println!("\n\n[with if let]");
    {

    }

}
