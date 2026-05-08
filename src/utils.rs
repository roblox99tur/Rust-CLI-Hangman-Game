use rand::prelude::SliceRandom;
use serde::{self, Deserialize, Serialize};
use serde_json;
use std::fs::{self, OpenOptions};
use std::io::{self, ErrorKind, Write};

const HISTORY: &str = "history.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub word: String,
    pub win: bool,
    pub attempts: usize,
}

impl History {
    pub fn new(word: String, win: bool, attempts: usize) -> Self {
        Self {
            word,
            win,
            attempts,
        }
    }
    pub fn print(&self) {
        println!("Word: {}", self.word);
        println!("Win: {:?}", self.win);
        println!("Attempts left: {}", self.attempts);
        println!();
    }
    pub fn show_history(history: &Vec<History>) {
        clear();
        if history.is_empty() {
            println!("No history to show.");
            empty();
            return;
        }

        for (i, entry) in history.iter().enumerate() {
            println!("Entry {}:", i + 1);
            entry.print();
        }
        empty();
    }
}

pub fn clear_history() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(HISTORY)?;
    file.write_all(b"")?;
    Ok(())
}

pub fn read_json_from_file() -> Result<Vec<History>, Box<dyn std::error::Error>> {
    let json_file = fs::read_to_string(HISTORY);
    match json_file {
        Ok(history_list) => {
            let history: Vec<History> = if history_list.is_empty() {
                Vec::new()
            } else {
                serde_json::from_str(&history_list)?
            };
            Ok(history)
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                fs::File::create_new(HISTORY)?;
                return Ok(Vec::new());
            }
            ErrorKind::PermissionDenied => return Err(error.into()),
            _ => return Err(error.into()),
        },
    }
}

pub fn write_json_to_file(history: &Vec<History>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(history)?;

    fs::write(HISTORY, json)?;

    Ok(())
}

pub fn get_random_word(words: &[String]) -> io::Result<String> {
    words
        .choose(&mut rand::thread_rng())
        .cloned()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Словарь пуст"))
}

pub fn load_words(path: &str) -> io::Result<Vec<String>> {
    let file = fs::read_to_string(path)?;
    Ok(file
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

pub fn show_word(vec: &mut Vec<char>, word: &String) -> bool {
    let mut flag = true;

    print!("Word: ");
    for c in word.chars() {
        if !vec.contains(&c.to_ascii_lowercase()) {
            print!("_ ");
            flag = false;
        } else {
            print!("{} ", c);
        }
    }
    println!();
    flag
}

pub fn guess(input: char, word: &String) -> bool {
    for c in word.chars() {
        if c.to_ascii_lowercase() == input.to_ascii_lowercase() {
            return true;
        }
    }

    false
}
pub fn input() -> String {
    let mut buff = String::new();

    match io::stdin().read_line(&mut buff) {
        Ok(_) => buff.trim().to_string(),
        Err(_) => String::new(),
    }
}
pub fn get_user_input() -> char {
    loop {
        print!("Enter your guess(char): ");
        io::stdout()
            .flush()
            .expect("Failed to clear the output buffer.");

        let buff = input();
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

pub fn clear() {
    println!("\x1B[2J\x1B[1;1H");
}

pub fn show_keyboard(vec: &Vec<char>) {
    let qwerty_alphabet: [char; 26] = [
        'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k',
        'l', 'z', 'x', 'c', 'v', 'b', 'n', 'm',
    ];

    for (i, &letter) in qwerty_alphabet.iter().enumerate() {
        if !vec.contains(&letter) {
            print!("{} ", letter);
        } else {
            print!("_ ",);
        }
        if i == 9 || i == 18 {
            println!();
        }
    }
    println!();
}

pub fn empty() {
    io::stdin().read_line(&mut String::new()).ok();
}
