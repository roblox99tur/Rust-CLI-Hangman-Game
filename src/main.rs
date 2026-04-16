use std::io::{self, Write};

fn main() {
    let attempts = 6;
    let word = String::from("guess");

    let mut vec: Vec<char> = Vec::new();

    for i in 0..attempts {
        println!("You have {}  attempts more", (attempts - (i + 1)));

        show_word(&mut vec, &word);

        let input = get_user_input();

        guess(&mut vec, &word, input);
    }
}

fn show_word(vec: &mut Vec<char>, word: &String) {
    print!("Word: ");
    for c in word.chars() {
        if !vec.contains(&c) {
            print!("_ ");
        } else {
            print!("{} ", c);
        }
    }
    println!();
}

fn guess(vec: &mut Vec<char>, word: &String, input: char) {
    for c in word.chars() {
        if c == input {
            vec.push(c);
            break;
        }
    }
}

fn get_user_input() -> char {
    loop {
        let mut buff = String::new();

        print!("Enter your guess(char): ");
        io::stdout()
            .flush()
            .expect("Не удалось очистить буфер вывода");

        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read line");

        let trimmed_buff = buff.trim();

        if trimmed_buff.len() != 1 {
            println!("Please enter a single char");
            continue;
        }

        let ch: char = match trimmed_buff.parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if ch.is_ascii_alphabetic() {
            return ch;
        }

        println!("Please do not enter non ascii symbols!");
    }
}
