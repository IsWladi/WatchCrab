use clap::Parser;
use std::path::Path;
use std::sync::mpsc::channel;

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

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

    println!("{:?}", args.events);

    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

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

    let recursive_mode = if args.recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    watcher
        .watch(path.canonicalize().unwrap().as_path(), recursive_mode)
        .unwrap();

    for event in rx {
        match event {
            Ok(event) => {
                let kind_str = if &args.events == &["all"] {
                    "all"
                } else if event.kind.is_access() {
                    "access"
                } else if event.kind.is_create() {
                    "create"
                } else if event.kind.is_modify() {
                    "modify"
                } else if event.kind.is_remove() {
                    "remove"
                } else {
                    continue;
                };

                let kind_str = String::from(kind_str);

                if kind_str == "all" {
                    println!("{:?}", event);
                } else if args.events.contains(&kind_str) == true {
                    println!("{:?}", event);
                }
            }

            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}
