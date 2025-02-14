# minigrep

## Overview
This project is a simple implementation of the grep command in Rust. It allows users to search for a query string within a file and displays matching lines. It also supports case-insensitive searching based on an environment variable.


## Project Structure
```bash
Practice_2-minigrep/            # Project root directory
│
├── src/                        # Source code directory
│   ├── main.rs                 # Main Rust program file
│   ├── lib.rs                  # Library containing core logic
│
├── Cargo.lock                  # Lock file for dependencies
├── Cargo.toml                  # Cargo project configurations
├── output.txt                  # Example output file
├── poem.txt                    # Input text file for testing
├── README.md                   # Project documentation
```

## Required Dependencies
No external dependencies required.

## Key Concepts Used
- File reading and handling
- Command-line arguments parsing
- Error handling using Result and Option
- Iterators and string manipulation
- Environment variables for case-insensitive searching
- Unit testing in Rust


## How to Run
1. Create a new Rust project:
```bash
cargo new minigrep
cd minigrep
```
2. Replace src/main.rs and src/lib.rs with the provided code.
4. Run the program with a query and file:
```bash
$Env:IGNORE_CASE=1; cargo run -- how poem.txt
```
or
```bash
cargo run -- how poem.txt
```


## Example Output
```bash
Searching for : to
In file       : poem.txt
Ignoring case : false
Are you nobody, too?
How dreary to be somebody!
```
or
```bash
Searching for : how
In file       : poem.txt
Ignoring case : true
How dreary to be somebody!
How public, like a frog
```
