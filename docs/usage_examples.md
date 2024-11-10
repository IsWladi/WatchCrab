# Usage Examples

WatchCrab provides flexible options for monitoring filesystem events and automating actions based on those events. Here are some example commands to help you get started.

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

**Unix**

By default, WatchCrab uses `sh -c` to execute commands on Unix-like systems (Linux, macOS). For example, to log each event:

```bash
watchcrab --path /path/to/directory --args "echo 'Event: {kind} -> Path: {path}'"
```

You can specify a different shell with the `--sh-cmd` flag, like `bash`:

```bash
watchcrab --path /path/to/directory --sh-cmd "bash -c" --args "echo 'Event: {kind} -> Path: {path}'"
```

**Windows**

On Windows, WatchCrab uses `cmd /C` by default to execute commands. To log each event in Windows:

```powershell
watchcrab --path C:\path\to\directory --args "echo Event: {kind} -> Path: {path}"
```

You can also specify a different shell, such as PowerShell, using `--sh-cmd`:

```powershell
watchcrab --path C:\path\to\directory --sh-cmd "powershell -Command" --args "Write-Output 'Event: {kind} -> Path: {path}'"
```

## 5. Flexible and Complex Command Execution

With WatchCrab, you can execute any shell command in response to filesystem events, allowing for high flexibility to adapt to your specific needs and creativity. The `--args` flag supports chaining multiple commands together, so you can build custom workflows that suit your tasks.

For example, you can log an event, copy the file to a backup folder, and update a log file in one command:

```bash
watchcrab --path /path/to/directory --events create --args "echo 'Event: {kind} -> Path: {path}' && cp -r {path} ./backup && echo 'log {kind} -> {path}' >> ./log.log"
```

The possibilities are endless. Here are a few ideas:
- **Compress files on creation**: Automatically compress files when they are created.
- **Trigger notifications**: Send desktop notifications or emails based on specific file events.
- **Run custom scripts**: Launch scripts for data processing, backups, or even API requests.

## 6. Asynchronous execution

If you want to run the shell command asynchronously, you can use --threads to specify the number of threads to use. For example, to run the command in four threads:

```bash
watchcrab --path /path/to/directory --args "sleep 5 && echo 'Event: {kind} -> Path: {path}'" --threads 4
```
**Note:** If you use the `--threads` flag with 1 thread, the command will run synchronously.
