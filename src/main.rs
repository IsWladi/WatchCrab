use std::path::Path;
use std::process::Command;

use clap::Parser;
use notify::Event;
use watchcrab::watch_sync;

/// Simple program to watch a directory for changes
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
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    match path {
        _ if path.exists() == false => {
            println!("Path does not exist");
            return;
        }
        _ if path.is_dir() == false => {
            println!("Path is not a directory");
            return;
        }
        _ => (),
    }

    // Closure to handle the events
    // Example of how to execute a command based on the event received
    // This just prints the path of the file that triggered the event with echo
    let f = |event: Event| {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args([
                    "/C",
                    "echo",
                    event.paths.iter().next().unwrap().to_str().unwrap(),
                ])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!(
                    "echo -n {}",
                    event.paths.iter().next().unwrap().to_str().unwrap()
                ))
                .output()
                .expect("failed to execute process")
        };

        let cmd_stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8 sequence");
        println!("echo Event: {:?}, Path: {:?}", event.kind, cmd_stdout);
    };

    watch_sync(&path, args.recursive, &args.events, f);
}
