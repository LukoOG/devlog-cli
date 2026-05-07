# devlog

A lightweight command-line tool for logging your daily development progress.

This project was built to practice Rust fundamentals, including CLI parsing, ownership, iterators, and modular project structure.

---

## Features

* Add logs from the terminal
* Attach one or multiple tags to logs
* Filter logs by tags
* Persistent storage using JSON
* Simple and fast CLI workflow

---

## Installation

### Using Cargo (recommended)

Requires Rust to be installed.

```bash
cargo install --git https://github.com/LukoOG/devlog-cli
```

---

## Usage

### Add a log

```bash
devlog add "learned about the borrow checker" --tag rust
```

Multiple tags:

```bash
devlog add "built CLI tool" --tag rust --tag cli
```

---

### List all logs

```bash
devlog list
```

---

### Filter logs by tag

```bash
devlog list --tag rust
```

Filter by multiple tags (AND logic):

```bash
devlog list --tag rust --tag cli
```

---

### Clear all logs

```bash
devlog clear
```

---

### Help

```bash
devlog help
```

---

## Example

```bash
devlog add "implemented JSON storage" --tag rust --tag backend
devlog add "fixed borrow checker issue" --tag rust

devlog list --tag rust
```

---

## Project Structure

```
src/
 ├── main.rs        # entry point
 ├── cli.rs         # argument parsing
 ├── commands.rs    # command handlers + Command enum
 ├── storage.rs     # file read/write logic
 └── models.rs      # data structures
```

---

## Notes

* Logs are stored locally in a `devlog.json` file
* An example of the file's structure is shown in example.json
* This is a learning project focused on Rust fundamentals, not production-grade tooling

---

## Future Improvements

* Better error handling with custom error types
* Improved CLI UX (formatting, colors)
* Additional filters (date, search)
* Optional export functionality

---

## License

MIT
