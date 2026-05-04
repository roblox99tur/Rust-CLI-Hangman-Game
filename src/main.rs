use rand::prelude::SliceRandom;
use std::fs;
use std::io::{self, Write};

#[derive(Debug)]
enum Game {
    Hangman,
}

struct Settings {
    game: Game,
}

fn main() -> io::Result<()> {
    let mut settings = Settings {
        game: Game::Hangman,
    };
    loop {
        clear();
        println!("1. Play");
        println!("2. Switch game ({:?})", settings.game);
        // println!("3. Settings");
        println!("3. Exit");

        match input_action() {
            1 => {
                hangman()?;
            }
            2 => {}
            // 3 => {
            //     settings_menu(&mut settings)?;
            // }
            3 => {
                break Ok(());
            }
            _ => {}
        }
    }
}

fn settings_menu(settings: &mut Settings) -> io::Result<()> {
    loop {
        clear();

        println!("1. ");
        println!("2. ");
        println!("3. Exit");

        match input_action() {
            1 => {}
            2 => {}
            3 => {
                break;
            }
            _ => {}
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

fn hangman() -> io::Result<()> {
    let hangman_stages: [&str; 7] = [
        r#"
                +-------+
                |       |
                |
                |
                |
                |
                ========="#,
        // 1: Голова
        r#"
                +-------+
                |       |
                |       O
                |
                |
                |
                ========="#,
        // 2: Туловище
        r#"
                +-------+
                |       |
                |       O
                |       |
                |
                |
                ========="#,
        // 3: Одна рука
        r#"
                +-------+
                |       |
                |       O
                |      /|
                |
                |
                ========="#,
        // 4: Обе руки
        r#"
                +-------+
                |       |
                |       O
                |      /|\
                |
                |
                ========="#,
        // 5: Одна нога
        r#"
                +-------+
                |       |
                |       O
                |      /|\
                |      /
                |
                ========="#,
        // 6: Повешен (6 ошибок)
        r#"
                +-------+
                |       |
                |       O
                |      /|\
                |      / \
                |
                ========="#,
    ];

    let mut tries: Vec<char> = Vec::new();

    let mut attempts: usize = 6;

    let words_list = load_words("words.txt")?;
    let word = get_random_word(&words_list)?;

    let mut vec: Vec<char> = Vec::new();

    while attempts > 0 {
        clear();

        println!("{}", hangman_stages[6 - attempts]);

        println!("You have {}  attempts more ", attempts);

        if show_word(&mut vec, &word) {
            println!("You win!");
            io::stdin().read_line(&mut String::new()).ok();

            break;
        }

        show_keyboard(&tries);

        let input = get_user_input();

        if !guess(&mut vec, &word, input) && !tries.contains(&input) {
            attempts -= 1;
        }

        tries.push(input);

        if attempts == 0 {
            clear();

            println!("{}", hangman_stages[6 - attempts]);

            show_word(&mut vec, &word);

            println!("You lose! The word was {}", word);
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

fn show_keyboard(vec: &Vec<char>) {
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
