#![allow(unused)]
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

/**
 * process panic
 * 1. print panic message
 * 2. unwind(roll back all called stack until a panic point)
 * 3. clean up the stack
 * 4. quit
 * cf: aborting => quit without unwinding
 * to configure put this at Cargo.toml
 * [profile.release]
 * panic = 'abort'
 */
fn panic_by_panic() {
    println!("before panic");
    // program shut down here
    panic!("crash and burn");

    println!("after panic");
}

fn panic_by_out_of_bound() {
    let v = vec![1, 2, 3];
    // this will call panic because `out of bounds` is vulnerable
    v[99];
    //RUST_BACKTRACE=1 cargo run will show backtrace(detail).
}

fn result_with_match() {
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
/**
 * .unwrap() try to do given action, call panic
 *  and send received Err when ERR
 */
fn result_with_unwrap() {
    let _hello_fd = File::open("hello.txt").unwrap();
}
/**
 * .expect() try to do given action, call panic
 *  and send received Err and ERROR_MSG defined by user when ERR
 */
fn result_with_expect() {
    let _hello_fd = File::open("hello3.txt").expect("no such files by jeanoza");
}

/**
 * Propagating Errors
 * Instead of handling error in function itself,
 * return Result<T, Err>
 * So, this function has no responsibility to handle error
 */
fn read_user_name_from_file(path: &mut String) -> Result<String, io::Error> {
    let username_file_fd = File::open(&path);
    // As I sent ref of path when open
    // The modification doesn't affect previus line
    // But path's value will be change after nextline(and also in main function)
    // *path += " after open by path";

    let mut username_fs = match username_file_fd {
        Ok(file) => file,
        Err(e) => return Err(e), // put explicitly `return` to terminate function here
    };

    let mut username = String::new();

    match username_fs.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/**
 * Using `?` operator, code will be more short
 * ? send T if suceed, else Err(e) like I wrote with match
 * Also, if any error detected at the ? operator, return directly Err(e)
 * So, it's useful when I wanna just passe T or Err without any modification
 *
 * Attention: `?` operator can be used only in the function
 * that returns Result<T, Err>, Option<T> or type implements `FromResidual`
 * Because `?` must know T type to return when success and also Err to return when fail
 * ex: we cannot use it in main which returns
 */
fn read_user_name_from_file_simple(path: &mut String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(&path)?.read_to_string(&mut username)?;
    Ok(username)
}
/**
 * fs::read_to_string do the stuffs(open, create string, read file and put buffer to that string)
 * that we did manually
 */
fn read_user_name_from_file_more_simple(path: &mut String) -> Result<String, io::Error> {
    fs::read_to_string(&path)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    let mut lines = text.lines(); // lines
    let next = lines.next()?; // first line
                              // let next = lines.next()?; // second line
                              // let next = lines.next()?; // third line
    next.chars().last()
}

fn main() {
    println!("* * * Error handle * * *");

    println!("\n[panic!]");
    // panic_by_panic();
    // panic_by_out_of_bound();

    println!("\n[Result<T, E>]");
    println!("\n- match expression");
    result_with_match();

    println!("\n- unwrap()");
    // result_with_match();

    println!("\n- expect([ERROR_MSG])");
    // result_with_expect();

    println!("\n[Propagating Errors]");
    let mut path = String::from("hello.txt");
    println!("\n- long version");
    println!("{:?}", read_user_name_from_file(&mut path));
    println!("\n- simple version");
    println!("{:?}", read_user_name_from_file_simple(&mut path));
    println!("\n- more simple simple version");
    println!("{:?}", read_user_name_from_file_more_simple(&mut path));

    println!("\n- with Option<T>");
    println!("{:?}", last_char_of_first_line("hello\nworld"));
    println!("{:?}", last_char_of_first_line("\nhi"));
}

// cf use '?' on main thanks to Result<(), Box<dyn Error>>
// use std::error::Error;
// use std::fs::{self, File};

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = fs::read_to_string("hello.txt")?;
//     println!("{greeting_file}");

//     Ok(())
// }
