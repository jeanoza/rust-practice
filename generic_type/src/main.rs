use std::cmp::PartialOrd;

fn largest_number_example() {
    let int_list = vec![34, 42, 21, 99, 64];
    let res = largest_number(&int_list);
    println!("The largest integer is {}", res);

    let float_list = vec![34, 6000, 21, 99, 64];
    let res = largest_number(&float_list);
    println!("The largest float is {}", res);

    let char_list = vec!['c', 'k', 't', 'm'];
    let res = largest_number(&char_list);
    println!("The largest(ordered by ascii) char is {}", res);
}

/**
 * add `PartialOrd` trait to comparer elements by using its references
 * :PartialOrd means that we accept only Generic T Type which implement PartialOrd
 * such as i32, float, char etc
 */
fn largest_number<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    println!("* * * Generic Type * * *");
    println!("[Removing Duplication by Extracting a Function]");
    println!("[Generic Types] by put <T>");
    largest_number_example();
}
