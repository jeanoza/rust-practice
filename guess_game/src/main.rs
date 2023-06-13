use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    /*
     * two diff way to concatenate string with variable
     */
    // let hello = random.;
    // let x = 5;
    // let y = 10;
    // println!("x = {x} and y + 2 = {}", y + 2);

    /*
     * bind manually &mut self (send ownership by first param??)
     * instead of using instance sent by previous function like 'secret_number'
     */
    // let mut thread_rng = rand::thread_rng();
    // let result = Rng::gen_range(&mut thread_rng, 1..=100);
    // println!("Test result: {result}");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("This is secret_number : {}", secret_number);

    loop {
        println!("Please input your guess.(Only number accepted)");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*
         * parse() conversion from String to type defined in left value
         * In this case(match with Result<OK, Err>),
         * `match` and `{} statement` stuff behave like try... catch in other language
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            // `_` means catch all error, not precised
            // So, in this scope, program ignore all errors
        };
        println!("You guessed: {guess}");

        /*
         * cmp returns Ordering enum
         * In this case(match with ()->Ordering),
         * `match` and `{} statement` behave like switch case
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }

        /*
         * cf/ diff between `->` and `=>`?
         * `->` : to define return type
         * `=>` : in match expression, to connect to body {} like above lines
         */
    }
}
