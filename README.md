# TicTacToe

A simple command-line Tic-Tac-Toe game written in Rust.

## Features
- Two-player mode
- Command-line interface
- Board updates after each move
- Win and draw detection

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)

### Running the Game
```sh
cargo run
```

### Running Tests
```sh
cargo test
```

## Project Structure
- `src/main.rs`: Main game logic and entry point

## How to Play
- Players take turns entering a number (1-9) to mark a position on the board.
- The first player to align three marks (horizontally, vertically, or diagonally) wins.
- If all positions are filled without a winner, the game ends in a draw.

## License
MIT
