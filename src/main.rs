use rand::prelude::SliceRandom;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
enum Game {
    Hangman,
}

struct Settings {
    attempts: u8,
    game: Game,
}

fn main() -> io::Result<()> {
    let mut settings = Settings {
        attempts: 6,
        game: Game::Hangman,
    };
    loop {
        clear();
        println!("1. Play");
        println!("2. Switch game ({:?})", settings.game);
        println!("3. Settings");
        println!("4. Exit");

        match input_action() {
            1 => {
                hangman(settings.attempts)?;
            }
            2 => {}
            3 => {
                settings_menu(&mut settings)?;
            }
            4 => {
                break Ok(());
            }
            _ => {}
        }
    }
}

fn settings_menu(settings: &mut Settings) -> io::Result<()> {
    loop {
        clear();

        println!("1. Change count of attempts ({})", settings.attempts);
        println!("2. ");
        println!("3. ");
        println!("4. Exit");

        match input_action() {
            1 => {
                change_attempts(settings)?;
            }
            2 => {}
            3 => {}
            4 => {
                break;
            }
            _ => {}
        }
    }

    Ok(())
}

fn change_attempts(settings: &mut Settings) -> io::Result<()> {
    loop {
        clear();

        print!("Print the number of attempts: ");
        io::stdout()
            .flush()
            .expect("Не удалось очистить буфер вывода");
        match input_action() {
            0 => {
                println!("Number of attempts must be greater than 0");
                continue;
            }
            other => {
                settings.attempts = other;
                break;
            }
        }
    }

    Ok(())
}

fn get_random_word(words: &[String]) -> io::Result<String> {
    words
        .choose(&mut rand::thread_rng())
        .cloned()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Словарь пуст"))
}

fn load_words(path: &str) -> io::Result<Vec<String>> {
    let file = fs::read_to_string(path)?;
    Ok(file
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
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

fn hangman(mut attempts: u8) -> io::Result<()> {
    let words_list = load_words("words.txt")?;
    // println!("Words list len: {}", words_list.iter().len());

    let word = get_random_word(&words_list)?;
    // println!("Word: {}", word);

    let mut vec: Vec<char> = Vec::new();

    while attempts > 0 {
        clear();

        println!("You have {}  attempts more ", attempts);

        if show_word(&mut vec, &word) {
            println!("You win!");
            io::stdin().read_line(&mut String::new()).ok();

            break;
        }

        let input = get_user_input();

        if !guess(&mut vec, &word, input) {
            attempts -= 1;
        }

        if attempts == 0 {
            println!("You lose!");
            io::stdin().read_line(&mut String::new()).ok();
        }
    }
    Ok(())
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
        if c.eq_ignore_ascii_case(&input) && !vec.contains(&c) {
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
