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
        println!("\n.push_str()");
        let mut s1 = String::from("foo");
        s1.push_str("bar"); // add &str
        dbg!(&s1);
        // actually, it seems assert_eq! compare only value bytes per btyes
        // So, it doesn't try to compare type(&str or String etc)
        assert_eq!(s1, "foobar");
    }
}
