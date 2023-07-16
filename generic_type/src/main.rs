fn largest_number_example() {
    let number_list = vec![34, 42, 21, 99, 64];
    let mut largest = &number_list[0];
    for number in &number_list {
        if number < largest {
            largest = number;
        }
    }
    println!("Largest number is {largest}");
}

fn main() {
    println!("* * * Generic Type * * *");
    println!("[Removing Duplication by Extracting a Function]");
    largest_number_example();
}
