mod db_utils;
mod services;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
}
