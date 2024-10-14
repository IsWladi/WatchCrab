use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use notify::Event;
use watchcrab::util::{parse_command, write_to_log_file, write_to_log_file_async};
use watchcrab::{watch_async, watch_sync};

/// Simple command line tool to watch a directory for changes and execute a command when an event is triggered
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to watch, it has to be an absolute path
    #[arg(short = 'p', long, default_value_t = String::from("./"))]
    path: String,

    /// Watch directories recursively, by default it will only watch the top level directory
    #[arg(short = 'r', long, default_value_t = false)]
    recursive: bool,

    /// Events to watch for, by default does not filter any events
    #[arg(short = 'e', long, num_args = 1.., value_delimiter = ' ', default_values = &["all"])]
    events: Vec<String>,

    /// Command to execute when an event is triggered, has to be a valid command. Can contain the '{path}' and '{kind}' placeholders
    #[arg(short = 'a', long, num_args = 1.., value_delimiter = ' ', default_values = &["default"])]
    args: Vec<String>,

    /// Output file to write logs to, by default it will print the logs to stdout
    #[arg(short, long)]
    output: Option<String>,

    /// Execute the command as an async closure, by default it will execute the command synchronously
    #[arg(long)]
    async_closure: bool,

    /// Number of threads to use when executing the closure asynchronously, by default it will use 4 threads
    #[arg(short = 't', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    match path {
        _ if path.exists() == false => {
            panic!("Path does not exist");
        }
        _ if path.is_dir() == false => {
            panic!("Path is not a directory");
        }
        _ => (),
    }

    // Select the write to log file function based on the async_closure flag
    let write_to_log = if args.async_closure {
        write_to_log_file_async
    } else {
        write_to_log_file
    };

    // Check if the output file is required and create it if it does not exist
    let mut output_file_path = PathBuf::new();
    let mut output_file_required = false;
    if args.output.is_some() {
        output_file_path = PathBuf::from(args.output.unwrap().as_str());

        // Create the file if it does not exist
        if output_file_path.exists() == false {
            std::fs::write(&output_file_path, "").expect("Unable to create log file");
        }

        output_file_path = output_file_path.canonicalize().unwrap(); // Get the absolute path

        output_file_required = true;
    }

    // Closure to handle the events
    // Example of how to execute a command based on the event received
    // By default just prints the event kind, path and the stdout of the command
    let f = move |event: Event| {
        let command = parse_command(
            &args.args,
            &event.paths.iter().next().unwrap().to_str().unwrap(),
            &format!("{:?}", event.kind).as_str(),
        );
        if args.args == ["default"] {
            let json_output = format!(
                r#"{{"Kind": "{}", "Path": "{}"}}"#,
                format!("{:?}", event.kind).as_str(),
                event.paths.iter().next().unwrap().to_str().unwrap()
            );
            if output_file_required {
                write_to_log(&output_file_path, &json_output);
            } else {
                println!("{}", json_output);
            }
        } else {
            // Execute the command and print the stdout and stderr
            let command_str = command.join(" ");
            let output = if cfg!(target_os = "windows") {
                Command::new("cmd")
                    .arg("/C")
                    .arg(command_str)
                    .output()
                    .expect("failed to execute command")
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(command_str)
                    .output()
                    .expect("failed to execute command")
            };

            let cmd_stdout = String::from_utf8_lossy(&output.stdout);
            let cmd_stderr = String::from_utf8_lossy(&output.stderr);

            let json_output = format!(
                r#"{{"stdout": "{}", "stderr": "{}"}}"#,
                cmd_stdout.trim(),
                cmd_stderr.trim()
            );

            if output_file_required {
                write_to_log(&output_file_path, &json_output);
            } else {
                println!("{}", json_output);
            }
        }
    };

    if args.async_closure {
        watch_async(&path, args.recursive, &args.events, f, args.threads);
    } else {
        watch_sync(&path, args.recursive, &args.events, f);
    }
}
