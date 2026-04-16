use std::io;

fn main() {
    let attempts = 6;
    let word = String::from("guess");

    let mut vec: Vec<char> = ['s'].to_vec();

    show_word(&mut vec, &word);
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
    println!();
}

fn input_guess() -> char {
    let mut buff = String::new();

    loop {
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read line");

        let trimmed_buff = buff.trim();

        if trimmed_buff.len() != 1 {
            print!("Please enter a single char");
            continue;
        }

        let buff: char = match trimmed_buff.parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        if buff.is_ascii_alphabetic() {
            buff;
        }

        println!("Please do not enter non ascii symbols!");
    }
}
