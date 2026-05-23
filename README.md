# Greping

A fast and simple command-line search tool built in Rust, inspired by `grep`.

## Features

- Search for words or patterns inside files
- Fast and memory safe
- Simple CLI interface
- Error handling with `Result`
- Case-sensitive search
- Lightweight and beginner-friendly Rust project

---

## Installation

### Clone the repository

```bash
git clone https://github.com/TaufiqSameer/greping.git
cd greping
```

---

## Build the project

```bash
cargo build
```

For optimized release build:

```bash
cargo build --release
```

---

## Usage

```bash
cargo run -- <query> <file_path>
```

### Example

```bash
cargo run -- Rust poem.txt
```

Example output:

```text
Rust:
safe, fast, productive.
```

---

## Project Structure

```text
greping/
│
├── src/
│   ├── main.rs
│   └── lib.rs
│
├── Cargo.toml
└── README.md
```

---

## How It Works

The program:

1. Reads command-line arguments
2. Opens the specified file
3. Reads file contents
4. Searches for matching lines
5. Prints matching results

---

## Error Handling

The application handles:

- Missing arguments
- Invalid file paths
- File reading errors

Example:

```text
Problem : No such file or directory
```

---

## Technologies Used

- Rust
- Cargo
- Standard Library (`std::fs`, `std::env`, `std::process`)

---

## Learning Goals

This project helped practice:

- Ownership & borrowing
- Structs and methods
- `Result` and error handling
- File handling
- Command-line argument parsing
- Modules and project organization

---

## Future Improvements

- Case-insensitive search
- Regex support
- Colored terminal output
- Recursive directory search
- Ignore hidden files
- Performance benchmarking

---

## Contributing

Pull requests are welcome. Feel free to fork the project and improve it.

---
