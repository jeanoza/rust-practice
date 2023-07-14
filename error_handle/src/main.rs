use std::{fs::File, io::ErrorKind};

fn main() {
    println!("* * * Error handle * * *");
    {
        println!("\n[panic!]");
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
        {
            let _v = vec![1, 2, 3];
            // this will call panic because `out of bounds` is vulnerable
            // v[99];
            //RUST_BACKTRACE=1 cargo run will show backtrace(detail).
        }
        println!("\n[Result<T, E>]");
        {
            println!("\n- match expression");
            {
                let hello_fd = File::open("hello.txt");
                // cf open with write permission
                // let hello_fd = OpenOptions::new().write(true).open("hello.txt");

                let _hello_fs = match hello_fd {
                    Ok(file) => file,
                    Err(error) => match error.kind() {
                        ErrorKind::NotFound => match File::create("hello.txt") {
                            Ok(fc) => fc,
                            Err(e) => panic!("Problem opening the file {:?}", e),
                        },
                        other_error => {
                            panic!("Problem opening the file {:?}", other_error);
                        }
                    },
                };
            }

            //.unwrap() try to do given action, call panic and send received Err when ERR
            println!("\n- unwrap()");
            {
                let _hello_fd = File::open("hello.txt").unwrap();
            }

            // .expect() try to do given action, call panic and send received Err and ERROR_MSG defined by user when ERR
            // more used by Rustacean rather than unwrap
            println!("\n- expect([ERROR_MSG])");
            {
                let _hello_fd = File::open("hello3.txt").expect("no such files by jeanoza");
            }
        }
    }
}
