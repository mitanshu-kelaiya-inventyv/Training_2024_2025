# Guessing Game

## Overview
This project is a simple number guessing game written in Rust. The program generates a random number between 1 and 100 and asks the user to guess it. It provides feedback if the guessed number is too high, too low, or correct.


## Project Structure
```bash
guessing_game/                   # Project root directory
│
├── src/                         # Source code directory
│   ├── main.rs                  # Main Rust program file  
│
├── Cargo.lock                   # Lock file for dependencies
├── Cargo.toml                   # Cargo project configurations
```

## Required Dependencies
1) rand

## Key Concepts Used
- Random number generation
- User input handling
- Looping and conditional logic
- Error handling with match
- Cargo package management


## How to Run
1. Create a new Rust project:
```bash
cargo new minigrep
cd minigrep
```
2. Replace src/main.rs and src/lib.rs with the provided code.
3. Build the project
```bash
cargo build
```
4. Run the game
```bash
cargo run
```
## Example Output
```bash
Your guess is high.
```
or
```bash
Your guess is low.
```
or
```bash
*** Correct guess, the number is 37 ***
```





