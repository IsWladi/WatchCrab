# WatchCrab

![» Crate](https://flat.badgen.net/crates/v/watchcrab)
![» Downloads](https://flat.badgen.net/crates/d/watchcrab)

`watchcrab` is a Rust-based command-line tool that monitors directories for filesystem events such as file creation, modification, and deletion. It allows you to execute shell commands when these events occur, making it ideal for automating file processing tasks or generating logs.

## Project Status

WatchCrab is currently in active development

## Features

- Watch directories for filesystem events.
- Execute custom shell commands when an event is triggered.
- Flexible and easy-to-use for automation workflows.
- JSON output for easy parsing and integration with other tools.

## Installation

### Binary Installation
1. No prerequisites are required to run the binary.
2. Go to the [releases page](https://github.com/IsWladi/WatchCrab/releases) and download the latest version for your operating system.
3. Extract the .zip file to a directory of your choice.
4. Add the binary to your PATH or provide the full path when running the command (or create an alias).

### Cargo Installation
Prerequisites:
- Rust (1.81.0 or later)
- Cargo

With Rust and Cargo installed, you can install `watchcrab` using the following command:

```bash
cargo install watchcrab
```

## Usage

To view all available options, use the `--help` flag:

```bash
watchcrab --help
```

For example, for starting to watch a directory for all filesystem events in the current directory recursively:

```bash
watchcrab --recursive
```

To view common usage examples, see the [usage examples](./docs/usage_examples.md) documentation.


## Additional Documentation

You can find the full documentation for watchcrab on [crates.io](https://crates.io/crates/watchcrab).

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/IsWladi/WatchCrab).
