# TicTacToe

A simple command-line Tic-Tac-Toe game written in Rust.

## Features
- Two-player mode
- Command-line interface
- Board updates after each move
- Win and draw detection

## How to Play

1. The game displays a 3x3 grid with positions numbered 1-9:
   ```
      |   |   
    1 | 2 | 3 
   ___|___|___
      |   |   
    4 | 5 | 6 
   ___|___|___
      |   |   
    7 | 8 | 9 
      |   |   
   ```

2. Players take turns entering a number (1-9) to place their mark
3. Player 1 uses "X" and Player 2 uses "O"
4. The first player to get three marks in a row (horizontally, vertically, or diagonally) wins
5. If all positions are filled without a winner, the game ends in a draw

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
