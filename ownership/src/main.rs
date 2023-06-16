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

        println!("s:{s} s2:{s2}");// save two value in stack 
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
        println!("{}",  s);

        let s1 = String::from("hello");
        let s2 = s1;
        // s1's value borrowed because of this assignation(moved)

        // println!("{s1}");



    }
}
