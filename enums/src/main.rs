fn main() {
    println!("* * * Enums * * *");
    println!("\n\n[Definition]");
    {
        println!("\n<Enum Values>");
        {
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
            fn _route(_ip_kind: IpAddrKind) {
                // println!("{:#?}", ip_kind);
                // dbg!(ip_kind);
            }

            #[derive(Debug)]
            struct Move {
                x: i32,
                y: i32,
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

            let message1 = Message::Move(Move { x: 4, y: 24 });
            let message2 = Message::Quit;
            let message3 = Message::ChangeColor(255, 255, 255);
            let message4 = Message::Write(hi_enum);
            message1.call();
            message2.call();
            message3.call();
            message4.call();
        }
        println!("\n<Option enum && Concept of Null in Rust>");
        {
            //exist like non-null
            let some_number = Some(5);
            let some_char = Some('e');

            //non-exist like null
            let absent_number: Option<i32> = None;

            dbg!(some_number, some_char, absent_number);

            let x = 5;
            let y = Some(8);
            let sum = x + y.unwrap_or_default();
            dbg!(sum);
        }
    }
    println!("\n\n[Pattern matching]");
    {
        println!("\n<ex1>");
        {
            #[derive(Debug)]
            enum UsState {
                Alabama,
                Alaska,
            }
            enum Coin {
                Penny,
                Nickel,
                Dime,
                Quarter(UsState),
            }

            fn value_in_cents(coin: Coin) -> u8 {
                match coin {
                    Coin::Penny => {
                        println!("Penny chosed");
                        1
                    }
                    Coin::Nickel => {
                        println!("Nickel chosed");
                        5
                    }
                    Coin::Dime => {
                        println!("Dime chosed");
                        10
                    }
                    Coin::Quarter(state) => {
                        println!("UsState:${:?}", state);
                        25
                    }
                }
            }

            dbg!(value_in_cents(Coin::Penny));
            dbg!(value_in_cents(Coin::Nickel));
            dbg!(value_in_cents(Coin::Dime));
            dbg!(value_in_cents(Coin::Quarter(UsState::Alabama)));
        }
        println!("\n<ex2: matching with Option<T>>");
        {
            fn plus_one(x: Option<i32>) -> Option<i32> {
                match x {
                    None => None,
                    Some(i) => Some(i + 1),
                }
            }

            //method I
            // fn add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
            //     match x {
            //         None => None,
            //         Some(i) => match y {
            //             None => None,
            //             Some(j) => Some(i + j),
            //         },
            //     }
            // }

            //method II
            // fn add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
            //     match x {
            //         None => None,
            //         Some(i) => match y {
            //             None => None,
            //             Some(j) => Some(i + j),
            //         },
            //     }
            // }

            //method III - refactoring of method II
            fn add(x: Option<i32>, y: Option<i32>) -> Option<i32> {
                match (x, y) {
                    (Some(i), Some(j)) => Some(i + j),
                    _ => None,
                }
            }

            let five = Some(5);
            dbg!(plus_one(five));
            dbg!(add(Some(5), Some(6)));
            dbg!(add(Some(5), None));
        }
    }
    println!("\n\n[with if let]");
    {
        let config_max = Some(24u8);
        let config_none: Option<u8> = None;
        if let Some(max) = config_max {
            println!("Max configured {}", max);
        }
        if let Some(element) = config_none {
            println!("if {}", element);
        } else {
            println!("else"); // must print else because element is None
        }
    }
}
