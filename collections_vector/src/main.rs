fn main() {
    println!("* * * Vector * * *");
    {
        println!("\n[Create]");
        {
            println!("\n- with new");
            let v: Vec<i32> = Vec::new(); // implicitly fix type with generic
            println!("{:?}", v);
        }
        {
            println!("\n- with vec!");
            // In this case, automatically, Rust find vector's generic types
            let v = vec![42, 0, -42];
            println!("{:?}", v);
        }
    }
    {
        println!("\n[Update]");
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        println!("{:?}", v);
    }
    {
        println!("\n[Read]");
        let v = vec![1,2,3,4,5];
        println!("{}",v[2]);
        println!("{}",v[2]);

        let v2 = vec![String::from("hello"), String::from("world")];
        println!("{}", v2[1]);
        println!("{}", v2[1]);

        let mut v3 = Vec::new();
        v3.push(String::from("hello"));
        println!("{}", v3[0]);
        println!("{}", &v3[0]);
        let first = &v3[0];
        // error because String doesn't implement `Copy trait`
        // That's why need to use .clone()
        // let first = v3[0];
        println!("{first}");



        println!("\nmatch using Option<> 1");
        // .get() method is more security as it returns Option<T> which can consider case None using match
        // with this method, we can avoid `index out of range` problem
        let first = v3.get(0);
        match first {
            Some(_first) => println!("In some: {_first}"),
            None => println!("<None>"),
        }
        println!("\nmatch using Option<> 2");
        let second = v3.get(1);
        match second {
            Some(_second) => println!("In some: {_second}"),
            None => println!("<None>"),
        }


        let mut v = vec![1, 2, 3, 4, 5];

        let first = v.get(0);
    
        // doesn't work because `first` point to `old v` which will be deallocated previus line
        // v.push(6);
        match first {
            Some(_first) => println!("The first element is: {_first}"),
            None => println!("None")
        }

    }
}
