use std::io::{self, Write};

fn main() {
    let mut attempts = 6;
    let word = String::from("guess");

    let mut vec: Vec<char> = Vec::new();

    while attempts > 0 {
        println!("You have {}  attempts more ", attempts);

        if show_word(&mut vec, &word) {
            println!("You win!");
            break;
        }

        let input = get_user_input();

        if !guess(&mut vec, &word, input) {
            attempts -= 1;
        }
    }
}

fn show_word(vec: &mut Vec<char>, word: &String) -> bool {
    let mut flag = true;

    print!("Word: ");
    for c in word.chars() {
        if !vec.contains(&c) {
            print!("_ ");
            flag = false;
        } else {
            print!("{} ", c);
        }
    }
    println!();
    flag
}

fn guess(vec: &mut Vec<char>, word: &String, input: char) -> bool {
    for c in word.chars() {
        if c == input && !vec.contains(&c) {
            vec.push(c);
            return true;
        }
    }

    false
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
