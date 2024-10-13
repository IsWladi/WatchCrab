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
1. Go to the [releases page](https://github.com/IsWladi/WatchCrab/releases) and download the latest version for your operating system.
2. Add the binary to your PATH or run it directly from the command line.

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

### Common Use Cases

#### 1. Monitor filesystem events in a specific directory

To start watching a directory for all filesystem events:

```bash
watchcrab --path /path/to/directory
```

#### 2. Recursively watch a directory and all its subdirectories

Enable recursive watching by adding the `--recursive` flag:

```bash
watchcrab --path /path/to/directory --recursive
```

#### 3. Filter events by type

You can specify which event types to monitor using the `--events` flag. For example, to only watch for file creation events:

```bash
watchcrab --path /path/to/directory --events create
```

To monitor multiple events, separate them with spaces:

```bash
watchcrab --path /path/to/directory --events create modify
```

#### 4. Execute a shell command when an event is triggered

The `--args` flag allows you to run a custom shell command when an event is detected. You can use placeholders in your command:

- `{kind}`: The type of event (e.g., create, modify, delete).
- `{path}`: The path to the file that triggered the event.

For example, to log each event:

```bash
watchcrab --path /path/to/directory --args "echo 'Event: {kind} -> Path: {path}'"
```

#### 5. Complex command execution

You can chain multiple commands together. For instance, to log an event, copy the file, and update a log file:

```bash
watchcrab --path /path/to/directory --events create --args "echo 'Event: {kind} -> Path: {path}' && cp -r {path} ./backup && echo 'log {kind} -> {path}' >> ./log.log"
```

#### 6. Experimental: Execute args with threads

You can enable the `--async-closure` flag to run the command in a separate thread. This can be useful for long-running commands or when you want to process multiple events concurrently.

```bash
watchcrab --path /path/to/directory --args "sleep 5 && echo 'Event: {kind} -> Path: {path}'" --async-closure --threads 4
```

## Additional Documentation

You can find the full documentation for watchcrab on [crates.io](https://crates.io/crates/watchcrab).

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/IsWladi/WatchCrab).
