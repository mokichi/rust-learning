use std::io;
use std::collections::HashMap;

fn main() {
    'outer: loop {
        println!("Please input number list.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut numbers = Vec::new();
        for s in input.trim().split(" ").collect::<Vec<&str>>() {
            let n: i32 = match s.parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("{:?}", e);
                    continue 'outer;
                }
            };
            numbers.push(n);
        }
        println!("You input: {:?}", numbers);

        println!("Mean: {}", mean(&numbers));
        println!("Median: {}", median(&numbers));
        println!("Mode: {:?}", mode(&numbers));
        break;
    }
}

fn mean(numbers: &Vec<i32>) -> f64 {
    let sum: i32 = numbers.iter().sum();
    sum as f64 / numbers.len() as f64
}

fn median(numbers: &Vec<i32>) -> f64 {
    let mut numbers: Vec<i32> = numbers.clone();
    numbers.sort();

    if numbers.len() % 2 == 0 {
        let i = numbers.len() / 2;
        let j = numbers.len() / 2 - 1;
        mean(&vec!(numbers[i], numbers[j]))
    } else {
        let i = numbers.len() / 2;
        numbers[i] as f64
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for n in numbers {
        let count = map.entry(n).or_insert(0);
        *count += 1;
    }

    let mut number_with_counts: Vec<(i32, usize)> = Vec::new();
    for (n, count) in map {
        number_with_counts.push((*n, count));
    }
    number_with_counts.sort_by_key(|t| t.1);
    number_with_counts.reverse();

    let max_count = number_with_counts[0].1;
    let mut numbers = Vec::new();
    for (n, count) in number_with_counts {
        if count == max_count {
            numbers.push(n);
        }
    }
    numbers.sort();
    numbers
}
