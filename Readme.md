# Hangman

A simple console-based Hangman game written in Rust.

## About

This is a classic word guessing game where you try to guess a hidden word by suggesting letters. You have 6 attempts to guess the word correctly before the hangman is complete.

## Features

- **Play the game** - Guess letters to uncover the hidden word
- **History tracking** - View your past games with results
- **Clear history** - Reset your game history

## Controls

1. Run the program
2. Select "1. Play" to start a new game
3. Enter single letters to guess the word
4. You have 6 incorrect attempts before you lose

## Requirements

- Rust (edition 2024)
- Dependencies: rand, serde, serde_json

## Running

```bash
cargo run
```

## Files

- `src/main.rs` - Main entry point with menu system
- `src/hangman.rs` - Game logic
- `src/utils.rs` - Utility functions (input handling, file I/O)
- `words.txt` - Word list for the game
- `history.json` - Stores game history

<!--## License

MIT-->
