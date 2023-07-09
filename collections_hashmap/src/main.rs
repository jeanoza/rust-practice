use std::collections::HashMap;

fn main() {
    println!("* * * HashMap * * *");
    {
        println!("\n[Create]");
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        //get value
        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        dbg!(score);
        assert_eq!(score, 10);

        //iterate in `for` loop
        println!("\n- iterate with for loop:");
        for (key, value) in &scores {
            println!("{key}: {value}");
        }

        //ownership
        println!("\n- Ownership:");
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);

        // dbg!(&field_name); // doesn't work ownership is moved to a previus map
        dbg!(&map);
    }
    {
        println!("[Update]");
        let mut scores = HashMap::new();

        let blue = String::from("Blue");

        scores.insert(blue.clone(), 10);
        println!("before update: {:?}", scores);
        assert_eq!(scores.get(&blue).copied(), Some(10));

        scores.insert(blue.to_owned(), 25); // if key exist => update, else => create
        println!("after update: {:?}", scores);
        assert_eq!(scores.get(&blue).copied(), Some(25));

        println!("\n- with .entry() and .or_insert() => Insert only when Key is not present:");
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50); // Blue exist, do nothing
        println!("{:?}", scores);

        println!("\n- pointer usage");
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            //.or_insert() returns mutable reference of current word's `Value`
            let count = map.entry(word).or_insert(0);
            //using dereference of this Value, increment count
            *count += 1;
        }
        println!("{:?}", map);
    }
    {
        println!("\n[Delete]");
        let mut map = HashMap::new();
        map.insert(String::from("Red"), 255);
        map.insert(String::from("Green"), 255);
        map.insert(String::from("Blue"), 255);
        map.insert(String::from("Alpha"), 1);
        println!("{:?}", map);
        let res_remove = map.remove(&String::from("Alpha"));
        println!("{:?}", map);
        let res_remove_entry = map.remove_entry(&String::from("Red"));
        println!("{:?}", map);

        println!("res_remove:{:?}", res_remove); // returns only removed Option<V>
        println!("res_remove_entry:{:?}", res_remove_entry); // returns removed Option<entry<K,V>>
    }
}
