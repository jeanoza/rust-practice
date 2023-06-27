#[derive(Debug)]
struct Rectangle {
    width:u32,
    height: u32,
}

//implement method
//self is like `this` in OOP
//we can also seperate method(ou function) with multiple `impl` blocks
//It will be useful for `generic types` and `trait` that will be discussed in Chapter10
impl Rectangle {
    //Associated Function like `static method` in OOP 
    fn square(size:u32) -> Self {
        Self {
            width: size,
            height:size
        }
    }
    fn area(&mut self) -> u32{
        self.height -=10; // we can also put mutable for self
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    println!("* * * Method syntax * * *");
    {
        let mut rect1 = Rectangle {
            width:30,
            height:50,
        };
        dbg!(&mut rect1.area());
        dbg!(&rect1.width());
        dbg!(&rect1);
    }
    println!("* * * Method syntax with more params* * *");
    {
        let rect1 = Rectangle {
            width:30,
            height:50,
        };
        let rect2 = Rectangle {
            width:10,
            height:40,
        };
        let rect3 = Rectangle {
            width:60,
            height:45,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }
    println!("* * * Associated Functions * * *");
    {
        let square = Rectangle::square(32); // like a constructor
        dbg!(square);
    }
}
