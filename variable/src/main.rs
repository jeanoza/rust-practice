fn main() {
    /*
     * `let`
     * can be mutable when following `mut`
     * give automatically type corresponding to right value
     */
    let mut x = 5; // need mut
    println!("the value of x is: {x}");
    x = 6; // but we cannot change data type
    println!("the value of x is: {x}");

    /*
     * `const`
     * doesn't give type => precise type by user
     * can't be mutable;
     * conventionally, use `UPPERCASE`
     */
    const Y: u32 = 8;
    println!("the value of Y is: {Y}");

    /*
     * Shadowing
     */
    let x = x + 1; // shadowing x => x + 1 (immutable)
    println!("the value of x is: {x}");
    {
        let x = "hello";
        // shadowing in scope x => "hello" (immutable)
        // Also, we can change `type` like that. => this is a diff thing with `mut`
        println!("the value of x in inner scope is: {x}");
    }
    println!("the value of x is: {x}");
    // not "hello" but 7!
    //because x = "hello" is valid in only previus local `scope`
}
