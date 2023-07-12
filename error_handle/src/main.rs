fn main() {
    println!("* * * Error handle * * *");
    {
        println!("\n[panic!]\n");
        {
            println!("before panic");
            // program shut down here
            // panic!("crash and burn");
            //1. print panic message
            //2. unwind(roll back all called stack until a panic point)
            //3. clean up the stack
            //4. quit
            //cf: aborting => quit without unwinding
            // to configure put this at Cargo.toml
            //[profile.release]
            //panic = 'abort'
            println!("after panic");
        }
    }
}
