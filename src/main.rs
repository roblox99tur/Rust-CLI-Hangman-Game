
use std::io;

mod hangman;
mod utils;

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
        utils::clear();
        println!("1. Play");
        println!("2. Switch game ({:?})", settings.game);
        // println!("3. Settings");
        println!("3. Exit");

        match utils::input_action() {
            1 => {
                hangman::hangman_game()?;
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
