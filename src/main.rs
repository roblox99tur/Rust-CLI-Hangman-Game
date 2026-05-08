mod hangman;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        utils::clear();
        println!("1. Play");
        println!("2. Show history");
        println!("3. Clear history");
        println!("4. Exit");

        let action: usize = match utils::input().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        match action {
            1 => {
                hangman::hangman_game()?;
            }
            2 => {
                utils::History::show_history(&utils::read_json_from_file()?);
            }
            3 => {
                utils::clear_history()?;
            }
            4 => {
                break Ok(());
            }
            _ => {}
        }
    }
}
