pub mod example;

use example::{first, second};
fn main() {
    first::first_fn();
    first::second_fn();
    second::first_fn();
    second::second_fn();
}
