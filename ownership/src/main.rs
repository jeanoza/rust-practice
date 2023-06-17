fn main() {
    /*
     * Ownership
     */
    println!("* * * Ownership * * *");
    println!("<main>");

    /*
     * Variable Scope
     * Local variable's capacity... same with other language
     */
    {
        println!("\n[Variable Scope]");
        //not yet s
        let s = "hello"; // &str is like char * in C/C++(saved in stack)
        let s2 = s;

        println!("s:{s} s2:{s2}"); // save two value in stack
        println!("test, {}", s.to_owned() + " hi &str");
        // s's memory will be free at the last scope
    }

    /*
     * String Type
     */
    {
        println!("\n[String Type]");
        let mut s = String::from("hello"); //Dynamic allocation using heaps
                                           // println!("test, {}", s + " hi String");
        s.push_str(", world!"); // push and re-allocate memory
        println!("{}", s);

        let s1 = String::from("hello");
        // let s2 = s1;
        // In another language like Java or JS,
        // s1's value would be `shallow copied` to s2
        // But in Rust, s1's value is moved to s2
        // Then origin value(s1) is `borrowed` => we cannot use anymore s1

        //`deep copy`, in Rust, we use clone(deep copy) for dynamic data(saved on `heap`)
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);

        //however, for static data, we don't need to use .clone()
        //because we know its size on compile time
        //ex:
        let s3 = "stack";
        let s4 = s3;
        println!("s3 = {}, s4 = {}", s3, s4);

        //cf/ Copy trait implementation
        // there is `trivial` copy saved on stack
        // ONLY the type that need not dynamic allocation can use `Copy` trait
        // u32, bool, f64, char, Tuple which doesn't contain dynmaic allocation
        // ex: (i32, i32) => OK, (i32, String) => NO!
        // but we will see more detail on chapter 10
    }

    /*
     * Function
     * Mechanism of passing value to a function is similar
     */
    {
        println!("\n[Functions]");
        let s = String::from("hello");

        // takes_ownership(s);
        // println!("{s}"); // doesn't work because s is moved to take_ownership function
        takes_ownership_void(s.clone());
        println!("after void function call: {s}"); // work!

        // ownership moving
        // give_ownership_str() -> println()
        println!("gives_ownership_str: {}", gives_ownership_str());

        // ownership moving
        //`s` -> echos_ownership() -> println()
        println!("echos_ownership: {}", echos_ownership(s));

        fn takes_ownership_void(str: String) {
            println!("takes_ownership: {}", str);
        }
        fn gives_ownership_str() -> String {
            // it's like `return String::from("yours");`
            // but in Rust, implicit return used more
            String::from("yours")
        }
        fn echos_ownership(str: String) -> String {
            str
        }
    }
}
