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
        println!("[Exercice 1: return median when given a list of integer]");
        dbg!(get_median(&[5]));
    }
    {
        println!("\n\n[Exercice 3: Employee books]");
        {
            let mut company: HashMap<String, Vec<String>> = HashMap::new();

            loop {
                let mut input = String::new();
                println!("
- Add [NAME] to <DEPARTMENT>
- List <DEPARTMENT>
- ListAll
"
                );

                io::stdin()
                    .read_line(&mut input)
                    .expect("Failted to readline");

                let cmds: Vec<&str> = input.split_whitespace().collect();

                if cmds.len() < 1 {
                    println!("Invalid input. Please try with:");
                    continue;
                }

                let action = cmds[0];

                match action {
                    "Add" => {
                        if cmds.len() != 4{
                            println!("Invalid input. Please try with:");
                            continue;
                        }
                        let name = cmds[1];
                        let department = cmds[3];
                        company
                            .entry(String::from(department))
                            .or_insert(Vec::new())
                            .push(String::from(name));

                        println!("Added {} to {}", name, department);
                    },
                    "ListAll" => {
                        if cmds.len() != 1{
                            println!("Invalid input. Please try with:");
                            continue;
                        }
                        let mut all_name_in_list:Vec<&String> = Vec::new();
                        for value in company.values().collect::<Vec<&Vec<String>>>() {
                            all_name_in_list.extend(value);
                        }

                        if all_name_in_list.is_empty() {
                            println!("List is Empty");
                            continue;
                        }
                        all_name_in_list.sort();
                        for name in all_name_in_list {
                            println!("{:?}", name)
                        }
                    },
                    "List" => {
                        if cmds.len() != 2{
                            println!("Invalid input. Please try with:");
                            continue;
                        }

                        let department = cmds[1];

                        let mut name_list = match company.get(department) {
                            Some(list) => list.clone(),
                            None => {
                                println!("Department not found");
                                continue;
                            }
                        };
                        name_list.sort();
                        for name in name_list {
                            println!("{:?}", name);
                        }

                    },
                    _ => {
                        println!("Input invalid");
                    }
                }
            }
        }
    }
}
