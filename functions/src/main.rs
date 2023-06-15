fn main() {
    /*
     * Functions 
     */
    println!("* * * Functions * * *");
    println!("<main>");

    /*
     * Parameters 
     */
    {
        println!("\n[Parameter]");
        another_function(5);
        print_labeled_measurement(42, 'c');
    }
    /*
     * Statements and Expression 
     * `Statements` do some action but not return a value
     * `Expressions` return value
     */
    {
        println!("\n[Statements and Expression]");
        // let x = (let y = 6); // doesn't work in Rust, because rust doesn't return value at assignement;
        // let mut x = 5;
        // let y = x = 6;  // y is ()

        //statement
        let y = {
            let x = 3;
            x + 100; // because of semicolon;
        }; // return nothing 
        //expression
        let y = {
            let x = 3;
            x + 100
        }; // return 4(x + 1)...
        println!("y is {y}");
    }

    /*
     * Function with Return Values
     */
    {
        println!("\n[Function with Return values]");
        println!("five() returns: {}", five());
        println!("plus_one(5) returns: {}", plus_one(5));

        //function with Return Values;
        fn five() -> i32 {
            5
        }

        fn plus_one(x:i32)-> i32{
            x + 1 // IMPORTANT: Do not put semicolon!! -> it will return `()` with `;` 
        }
    }
}


fn another_function (x:i32){
    println!("<another_function>");
    println!("x is {x}");
}

fn print_labeled_measurement(value:i32, unit_label:char) {
    println!("The measurement is: {}{}", value, unit_label);
}
