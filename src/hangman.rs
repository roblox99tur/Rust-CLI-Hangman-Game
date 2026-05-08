use crate::utils::{self, read_json_from_file};

pub fn hangman_game() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut history: Vec<utils::History> = read_json_from_file()?;

    let mut inputted_chars: Vec<char> = Vec::new();

    let mut attempts: usize = 6;

    let words_list = utils::load_words("words.txt")?;
    let word = utils::get_random_word(&words_list)?;

    let mut status: bool = false;
    // Game loop

    while attempts > 0 {
        utils::clear();

        println!("{}", hangman_stages[6 - attempts]);

        println!("You have {}  attempts more ", attempts);

        if utils::show_word(&mut inputted_chars, &word) {
            println!("You win!");
            utils::empty();
            status = true;
            break;
        }

        utils::show_keyboard(&inputted_chars);

        let input = utils::get_user_input().to_ascii_lowercase();

        if !utils::guess(input, &word) && !inputted_chars.contains(&input) {
            attempts -= 1;
        }

        if !inputted_chars.contains(&input) {
            inputted_chars.push(input.to_ascii_lowercase());
        }

        if attempts == 0 {
            utils::clear();

            println!("{}", hangman_stages[6 - attempts]);

            utils::show_word(&mut inputted_chars, &word);

            println!("You lose! The word was {}", word);
            utils::empty();
        }
    }

    history.push(utils::History::new(word, status, attempts));

    utils::write_json_to_file(&history)?;

    Ok(())
}
