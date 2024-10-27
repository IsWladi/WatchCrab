use std::path::{Path, PathBuf};
use std::process::Child;
use std::sync::Arc;

use clap::Parser;
use notify::Event;

#[cfg(target_family = "unix")]
use watchcrab::util::command_exec_unix as command_exec;

#[cfg(target_family = "windows")]
use watchcrab::util::command_exec_windows as command_exec;

use watchcrab::util::{parse_command, write_to_log_file, write_to_log_file_async};
use watchcrab::Watch;

/// Simple command line tool to watch a directory for changes and execute a command when an event is triggered
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to watch
    #[arg(short = 'p', long, default_value_t = String::from("./"))]
    path: String,

    /// Watch directories recursively, by default it will only watch the top level directory
    #[arg(short = 'r', long, default_value_t = false)]
    recursive: bool,

    /// Events to watch for, by default does not filter any events
    #[arg(short = 'e', long, num_args = 1.., value_delimiter = ' ', default_values = &["all"])]
    events: Vec<String>,

    /// shell command that will receive the --args as a string, by default it will use "sh -c" or "cmd /C" based on the OS
    #[arg(short = 's', long)]
    sh_cmd: Option<String>,

    /// Arguments to be passed to the shell command, by default it will not pass any arguments
    #[arg(short = 'a', long, num_args = 1.., value_delimiter = ' ')]
    args: Option<Vec<String>>,

    /// Number of threads to execute the command in, by default it will execute the command in the main thread
    #[arg(short = 't', long, default_value_t = 1)]
    threads: usize,

    /// Output file to write logs to, by default it will print the logs to stdout
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let mut args = Args::parse();

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

    // Validate the shell command
    let cmd_required = args.sh_cmd.is_some();
    if cmd_required && args.args.is_none() {
        panic!("Arguments are required when --sh-cmd is provided");
    } else if cmd_required == false && args.args.is_some() {
        // If args are provided but the shell command is not, then infer the shell command based on the OS
        args.sh_cmd = if cfg!(target_os = "windows") {
            Some("cmd /C".to_string())
        } else {
            Some("sh -c".to_string())
        };
    }
    let sh_cmd_split: Vec<String> = args
        .sh_cmd
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.to_string())
        .collect();
    if sh_cmd_split.len() < 1 {
        panic!("Invalid shell command, should be in the format: <shell> <command> for example: /bin/bash -c");
    }

    // Select the write to log file function based on the threads flag
    let write_to_log = if args.threads > 1 {
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
    let f = Arc::new(Box::new(move |event: Event| {
        // Get the path of the file that triggered the event
        let path = event.paths.iter().next().unwrap().to_str().unwrap();
        let clean_path = if cfg!(target_os = "windows") {
            path.replace(r"\\?\", "")
        } else {
            path.to_string()
        };

        // By default just prints the event kind and path of the file that triggered the event
        if !cmd_required && args.args.is_none() {
            let json_output = format!(
                r#"{{"Kind": "{}", "Path": "{}"}}"#,
                format!("{:?}", event.kind).as_str(),
                clean_path
            );
            if output_file_required {
                write_to_log(&output_file_path, &json_output);
            } else {
                println!("{}", json_output);
            }
            // If args are provided, then parse the command and execute it
        } else {
            let parsed_args = parse_command(
                args.args.clone().unwrap().as_ref(),
                &clean_path,
                &format!("{:?}", event.kind).as_str(),
            );

            // Execute the command and print the stdout and stderr
            let args_str = parsed_args.join(" ");
            let child: Child = command_exec(&sh_cmd_split, args_str);

            if let Ok(output) = child.wait_with_output() {
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
            } else {
                eprintln!("Command terminated unexpectedly.");
            }
        }
    }) as Box<dyn Fn(Event) + Send + Sync + 'static>);

    if cfg!(target_os = "windows") {
        println!("Warning: In Windows, the graceful shutdown is not supported, so the commands can be abruptly terminated");
        if args.threads > 1 {
            println!("Warning: Be cautious when using multiple threads in Windows, as the commands can be abruptly terminated");
        }
    }

    let watchcrab_watch = Watch::new(&path, args.recursive, &args.events, f, args.threads);
    let result = watchcrab_watch.start();

    match result {
        Ok(_) => {
            println!("Exiting watch from main");
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
