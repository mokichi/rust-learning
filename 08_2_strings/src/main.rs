use std::io;

fn main() {
    println!("Please input english words");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let words: Vec<&str> = input.trim().split(" ").collect();
    println!("You input: {:?}", words);

    for word in words {
        println!("{} -> {}", word, pig_latin(word));
    }
}

fn pig_latin(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    if chars.len() == 0 {
        return String::from("")
    }

    match chars[0] {
        'a' | 'i' | 'u' | 'e' | 'o' => format!("{}-hay", word),
        _ => {
            if word.len() > 1 {
                format!("{}-{}ay", &word[1..], &word[0..1])
            } else {
                format!("{}ay", word)
            }
        }
    }
}
