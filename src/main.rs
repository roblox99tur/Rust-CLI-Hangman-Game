use std::io::{self, Write, stdin};

fn main() {
    let attempts = 6;
    let game = String::from("hangman");
    let word = String::from("guess");

    let mut vec: Vec<char> = Vec::new();

    loop {
        clear();
        println!("1. Play");
        println!("2. Exit");
        println!("3. Switch game ({})", game);

        match input_action() {
            1 => {
                hangman(&mut vec, &word, attempts);
            }
            2 => {
                break;
            }
            3 => {}
            _ => {}
        }
    }
}

fn input_action() -> u8 {
    let mut buff = String::new();

    match io::stdin().read_line(&mut buff) {
        Ok(_) => {
            let input: u8 = match buff.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            input
        }
        Err(_) => 0,
    }
}

fn hangman(vec: &mut Vec<char>, word: &String, mut attempts: u8) {
    while attempts > 0 {
        clear();

        println!("You have {}  attempts more ", attempts);

        if show_word(vec, &word) {
            println!("You win!");
            io::stdin().read_line(&mut String::new()).ok();
            break;
        }

        let input = get_user_input();

        if !guess(vec, &word, input) {
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

fn clear() {
    println!("\x1B[2J\x1B[1;1H");
}
