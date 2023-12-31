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
    /*
     * Returning ownership of params 
     */
    {
        println!("\n[Returning ownership of params]");
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);
    
        println!("The length of '{}' is {}.", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() returns the length of a String

            (s, length)
        }
    }

    /*
     * Reference borrowing / Not owning
     * Instead of moving ownership, send its reference to function
     */
    {
        println!("\n[Reference borrowing]");
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);


        // In this function, s:&String doesn't have onwership
        fn calculate_length(s:&String) -> usize {
            s.len()
        }
    }
    /*
     * Mutable Reference
     * to add `&mut` to function's param
     * function call line
     * function defined line
     */
    {
        println!("\n[Mutable Reference]");
        let mut s = String::from("hello");

        //errors
        // change_str(&s);
        // fn change_str(s:&String) {
        //     s.push_str(", world");
        // }

        change_str(&mut s);
        fn change_str(s:&mut String) {
            s.push_str(", world");
        }
        println!("{s}");

        
        /*
         * Restriction
         * Impossible to put mutable reference twice at the same time
         * This restrict is to avoid `data race`
         * BUT => we can anyway use `immutable reference` multiple time
         */
        {
            println!("\n[restriction]");
            let mut s = String::from("hello");
            // let r1 = &mut s;
            // let r2 = &mut s; // error
            let r1 = &s;
            let r2 = &s;
            println!("{} {}", r1, r2);
        }

        /*
         * Dangling Reference
         * This case is invalid because in Rust, all memory will be freed at the last line in scope.
         * So, returning reference of a allocated variable in function can't be valid.
         */
        {
            println!("\n[Dangling Reference]");
            // let reference_to_nothing = dangle();
            // fn dangle() -> &String {
            //     let s = String::from("Dangle");
            //     &s
            // }
            let valid = no_dangle();
            fn no_dangle() -> String {
                let s = String::from("No dangle");
                s
            }
            println!("{}", valid);
        }

        /*
         * Recap
         * At any given time, you can have either one mutable reference or any number of immutable references.
         * References must always be valid.
         */

        /*
         * Slice Type
         */
        {
            println!("\n[Slice Type]");

            let s = String::from("hello world");
            let word_length = first_word_length(&s);
            let word = first_word_str(&s);
            let word2 = first_word_str(&s[..]);
            println!("word:\t\t{}", word);
            println!("word2:\t\t{}", word2);
            

            // WE CANNOT exec this line because of conflict between mutable vs immutable
            // s.clear();
            // println!("word_length:\t{}", word_length);
            // println!("word:\t\t{}", word);

            /*
             * returns index of the end of the first word
             */
            fn first_word_length(s: &String) -> usize {
                let bytes = s.as_bytes();
            
                for (i, &item) in bytes.iter().enumerate() {
                    // println!("i:{} &item:{}", i, item);
                    // i => index, item => ascii num(u8) of each element
                    if item == b' ' {
                        return i;
                    }
                }
            
                s.len()
            }

            /*
             * returns &str manipulating slice of &String
             * use `&str` instead of `&String` in param is more flexible && general
             *  => We can send &str, &String and also &str[..] -slicing
             */
            fn first_word_str(s:&str) -> &str{
                let bytes = s.as_bytes();
                for(i, &item) in bytes.iter().enumerate() {
                    if item == b' ' {
                        return &s[0..i];
                    }
                }

                &s[..]
            }

            /*
             *  String Slice
             *  syntax: &REF[START_IDX..END_IDX] => END exclude like another lang
             */
            {
                println!("\n[String slice]");
                let s = String::from("hello world");

                let hello = &s[0..5]; // or [..5]
                let world = &s[6..11]; // or [6..]

                println!("{}\n{}", hello, world);
            }

            /*
             * &str slice
             */
            {
                println!("\n[&str slice]");
                let s = "bonjour tout le monde";
                println!("{}", &s[0..7]);
            }

            /*
             * other slice
             */
            {
                println!("\n[Other slices]");

                let a = [1,2,3,4,5];
                let slice = &a[1..3]; // [2,3]

                assert_eq!(slice, &[2,3]);
            }
        }
    }
}
