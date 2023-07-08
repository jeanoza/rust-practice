fn main() {
    println!("* * * String * * *");
    {
        println!("\n[Create]");
        println!("\n- String::new()");
        let mut s1 = String::new(); // created empty string
        s1 += "hi";
        dbg!(&s1);
        assert_eq!(s1, "hi".to_string());

        println!("\n- .to_string()");
        let data = "this is &str";
        let s2 = data.to_string();
        dbg!(&s2);
        assert_eq!(s2, String::from("this is &str"));

        println!("\n- String::from()");
        let s3 = String::from("with from");
        dbg!(&s3);
        assert_eq!(s3, "with from".to_string());
    }
    {
        println!("\n[Update]");
        println!("\n.push_str(&str)");
        let mut s1 = String::from("foo");
        s1.push_str("bar"); // add &str
        dbg!(&s1);
        // actually, it seems assert_eq! compare only value bytes per btyes
        // So, it doesn't try to compare type(&str or String etc)
        assert_eq!(s1, "foobar");

        println!("\n.push(char)");
        let mut s2 = String::from("lo");
        s2.push('l');
        dbg!(&s2);
        assert_eq!(s2, "lol");

        println!("\n- operator '+'");
        let s_origin = String::from("hello, ");
        let s_to_add = String::from("world!");
        // <first> has to be String and  <second> &str.
        // '+' operator use .add() in String
        // s3 = s1 + &s2 => s3 = s1.add(&s2);
        // in add function, s2(&String) will be casted to a &str
        // more info =>  Chapter 15
        // in this logic, there is no copy(or clone)
        // s1 use its self, s2 is reference.
        // s1 is not valid, and s3 use s1's memory and s2's ref.
        let s_result = s_origin + &s_to_add;
        dbg!(&s_result);
        assert_eq!(&s_result, "hello, world!");
        {
            println!("\n- Concatenate with +:");
            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            let s = s1 + "-" + &s2 + "-" + &s3;
            dbg!(&s);
            // dbg!(&s1, &s2, &s3); s1 is no longer valid
        }
        {
            //format! uses references of String => also, it's more readable then '+'
            println!("\n- Concatenate with format! macro:");
            let s1 = String::from("tic");
            let s2 = String::from("tac");
            let s3 = String::from("toe");

            let s = format!("{s1}-{s2}-{s3}");
            dbg!(&s);
            dbg!(&s1, &s2, &s3); // s1 is still valid
        }
    }
    {
        println!("\n[Indexing]");
        let s1 = String::from("hello");
        let sliced = &s1[0..4];
        dbg!(s1.as_bytes()[0]);
        dbg!(s1.chars().nth(0)); // returns Option<char>
        assert_eq!(s1.chars().nth(0), Some('h'));
        dbg!(sliced);
        assert_eq!(sliced, "hell");

        let hello = "Здравствуйте";
        let s = &hello[0..4];
        dbg!(s);
    }
    {
        println!("\n[Iterate over String]");
        for c in "hello".chars() {
            println!("char :{c}");
        }
        for b in "hello".bytes() {
            println!("bytes(ascii): {b}");
        }
    }
}
