use std::{collections::HashMap, io};

fn get_median(list: &[i32]) -> f32 {
    let mut v = list.to_vec();
    let len = v.len();
    let mid = len / 2;

    v.sort();

    if len % 2 == 0 {
        (v[mid - 1] + v[mid]) as f32 / 2.0
    } else {
        v[mid] as f32
    }
}

fn main() {
    println!("* * * Exercices for collection * * *");
    {
        println!("[Exercice 1: return median when given a list of integer");
        dbg!(get_median(&[5]));
    }
    {
        println!("[Exercice 3: Employee books");
        {
            let mut company: HashMap<String, Vec<String>> = HashMap::new();

            loop {
                let mut input = String::new();

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failted to readline");

                let cmds: Vec<&str> = input.split_whitespace().collect();

                if cmds.len() < 2 {
                    println!(
                        "Invalid input. Please try with:
    - Add [NAME] to <DEPARTMENT>
    - List <DEPARTMENT>
"
                    );
                    continue;
                }

                let action = cmds[0];
                let name = cmds[1];
                let department = cmds[3];

                match action {
                    "Add" => {
                        company
                            .entry(String::from(department))
                            .or_insert(Vec::new())
                            .push(String::from(name));

                        println!("Added {} to {}", name, department);
                    }
                    "List" => {}
                    _ => {
                        println!("Input invalid");
                    }
                }
            }
        }
    }
}
