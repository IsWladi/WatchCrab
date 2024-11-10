# WatchCrab

![» Crate](https://flat.badgen.net/crates/v/watchcrab)
![» Downloads](https://flat.badgen.net/crates/d/watchcrab)

`watchcrab` is a Rust-based command-line tool that monitors directories for filesystem events like file creation, modification, and deletion. It triggers shell commands in response to these events, making it ideal for automating file-processing workflows, generating logs, or integrating with larger automation systems.

## Project Status

WatchCrab is now fully featured, and the project will be maintained to address any issues or bugs. New features may be added if deemed necessary. If you encounter any problems or have suggestions, please open an issue.

## Features
- **Directory Monitoring**: Monitors directories for events, such as file creation, modification, and deletion.
- **Automated Command Execution**: Executes customizable shell commands in response to events.
- **Asynchronous Event Handling**: Optionally handle events asynchronously for higher performance.
- **JSON Output**: Outputs events in JSON for easier parsing and integration with other tools.
- **Graceful Shutdown**: Waits for ongoing tasks to complete before termination, preventing data loss.
- **Cross-Platform**: Compatible with Unix-like systems (Linux, macOS) and Windows.
- **Command Logging**: Optionally logs stdout and stderr of commands for debugging.

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

To view more usage examples, see the [usage examples](./docs/usage_examples.md) documentation.


## Additional Documentation

You can find the full documentation for watchcrab on [crates.io](https://crates.io/crates/watchcrab).

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/IsWladi/WatchCrab).
