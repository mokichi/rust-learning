use std::io;
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    loop {
        println!("Please input command. Empty line exits this program.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let words: Vec<&str> = input.trim().split(" ").collect();
        if words.len() != 4 {
            break;
        }

        if words[0] == "Add" && words[2] == "to" {
            let department = String::from(words[3]);
            let employees = map.entry(department).or_insert(Vec::<String>::new());
            employees.push(String::from(words[1]));
        }
        println!("{:?}", &map);
    }

    let mut departments: Vec<&String> = map.keys().collect();
    departments.sort();
    for department in departments {
        let mut employees = map[department].clone();
        employees.sort();
        println!("{} => {:?}", department, employees);
    }
}
