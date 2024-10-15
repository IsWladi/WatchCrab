# Common Use Cases

## 1. Monitor filesystem events in a specific directory

To start watching a directory for all filesystem events:

```bash
watchcrab --path /path/to/directory
```

## 2. Recursively watch a directory and all its subdirectories

Enable recursive watching by adding the `--recursive` flag:

```bash
watchcrab --path /path/to/directory --recursive
```

## 3. Filter events by type

You can specify which event types to monitor using the `--events` flag. For example, to only watch for file creation events:

```bash
watchcrab --path /path/to/directory --events create
```

To monitor multiple events, separate them with spaces:

```bash
watchcrab --path /path/to/directory --events create modify
```

## 4. Execute a shell command when an event is triggered

The `--args` flag allows you to run a custom shell command when an event is detected. You can use placeholders in your command:

- `{kind}`: The type of event (e.g., create, modify, delete).
- `{path}`: The path to the file that triggered the event.

For example, to log each event:

```bash
watchcrab --path /path/to/directory --args "echo 'Event: {kind} -> Path: {path}'"
```

The `--args` string is passed directly to the shell, so you can use any valid shell command, by default it uses `sh -c` or `cmd /C` depending on the OS.

You can choose the shell to use with the `--sh-cmd` flag:

```bash
watchcrab --path /path/to/directory  --sh-cmd "bash -c" --args "echo 'Event: {kind} -> Path: {path}'"
```

## 5. Complex command execution

You can chain multiple commands together. For instance, to log an event, copy the file, and update a log file:

```bash
watchcrab --path /path/to/directory --events create --args "echo 'Event: {kind} -> Path: {path}' && cp -r {path} ./backup && echo 'log {kind} -> {path}' >> ./log.log"
```

## 6. Asynchronous execution

If you want to run the shell command asynchronously, you can use --threads to specify the number of threads to use. For example, to run the command in four threads:

```bash
watchcrab --path /path/to/directory --args "sleep 5 && echo 'Event: {kind} -> Path: {path}'" --threads 4
```
**Note:** If you use the `--threads` flag with 1 thread, the command will run synchronously.
