# WatchCrab

`watchcrab` is a Rust-based command-line tool that monitors directories for filesystem events such as file creation, modification, and deletion. It allows you to execute shell commands when these events occur, making it ideal for automating file processing tasks or generating logs.

## Features

- Watch directories for filesystem events.
- Execute custom shell commands when an event is triggered.
- Flexible and easy-to-use for automation workflows.
- Use as a standalone CLI tool or integrate directly into your Rust project for more control.

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

## Additional Documentation

You can find the full documentation for watchcrab on [crates.io](https://crates.io/crates/watchcrab).

## Contributing

Contributions are welcome! Please feel free to open issues or submit pull requests on the [GitHub repository](https://github.com/IsWladi/WatchCrab).
