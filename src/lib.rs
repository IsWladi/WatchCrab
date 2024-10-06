use std::path::Path;
use std::sync::mpsc::channel;

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

/// Watch a directory for changes synchronously
///
/// # Arguments
/// * `path` - Path to the directory to watch
/// * `recursive` - Watch directories recursively
/// * `events` - Events to watch for
///     e.g. ["all"] or ["access", "create", "modify", "remove"]
/// * `f` - Function to handle the events, it receives an `Event` object
pub fn watch_sync(path: &Path, recursive: bool, events: &Vec<String>, f: fn(Event)) {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    let recursive_mode = if recursive {
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
                let kind_str = if events == &["all"] {
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

                if kind_str == "all" || events.contains(&kind_str) == true {
                    f(event);
                }
            }

            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}

pub fn watch_async() {}
