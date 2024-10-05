use clap::Parser;
use std::path::Path;
use std::sync::mpsc::channel;

use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

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

    /// Events to watch for, by default it will watch for all events defined in the enum `Event`
    #[arg(short = 'e', long, num_args = 1.., value_delimiter = ' ', default_values = &["create", "remove"])]
    events: Vec<String>,
}

#[derive(Debug)]
enum Event {
    Create,
    Remove,
}

impl Event {
    fn from_str(s: &str) -> Option<Event> {
        match s {
            "create" => Some(Event::Create),
            "remove" => Some(Event::Remove),
            _ => None,
        }
    }

    fn matches_event(kind: &EventKind, event: &Event) -> bool {
        match (kind, event) {
            (EventKind::Create(_), Event::Create) => true,
            (EventKind::Remove(_), Event::Remove) => true,
            _ => false,
        }
    }
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

    let user_events: Vec<Event> = args
        .events
        .iter()
        .filter_map(|e| Event::from_str(e))
        .collect();

    for event in rx {
        match event {
            Ok(event) => {
                // println!("{:?}", event);
                for kind in &user_events {
                    // println!("KIND FOR DEBUG: {:?} ", kind);
                    if Event::matches_event(&event.kind, &kind) {
                        println!("Event: {:?}, Paths: {:?}", kind, event.paths);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}
