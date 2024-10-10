# WatchCrab

`watchcrab` is a Rust-based command-line tool that monitors directories for filesystem events such as file creation, modification, and deletion. It allows you to execute shell commands when these events occur, making it ideal for automating file processing tasks or generating logs.

## Features

- Watch directories for filesystem events.
- Execute custom shell commands when an event is triggered.
- Flexible and easy-to-use for automation workflows.
- Use as a standalone CLI tool or integrate directly into your Rust project for more control.

## Usage

To start watching a directory for file events, use the following command:

```bash
watchcrab --path /path/to/directory --args "echo {path}"
```
## Documentation
You can find the full documentation for watchcrab on [crates.io](https://crates.io/crates/watchcrab).
