use std::sync::mpsc::channel;
use std::{path::Path, sync::Arc};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use threadpool::ThreadPool;

/// Watch a directory for changes synchronously
///
/// # Arguments
/// * `path` - Path to the directory to watch
/// * `recursive` - Watch directories recursively
/// * `events` - Events to watch for
///     e.g. ["all"] or ["access", "create", "modify", "remove"]
/// * `f` - Function to handle the events, it receives an `Event` object
///
/// # Examples
///
/// **Print all filesystem events in the current directory**
///
/// ```no_run
/// use std::path::Path;
/// use notify::Event;
/// use watchcrab::watch::watch_sync;
///
/// let path = Path::new("./"); // Watch the current directory, you can change this to any path
/// let recursive = false; // Watch only the top level directory, you can change this to true
/// let events = vec!["all".to_string()]; // Watch all events, you can change this to ["access", "create", "modify", "remove"] or any combination of these, at least one is required
/// let f = |event: Event| {
///    println!("{:?}", event); // Print the event, you can replace this with your own logic
/// };
///
/// watch_sync(&path, recursive, &events, f);
/// ```

pub fn watch_sync(path: &Path, recursive: bool, events: &Vec<String>, f: impl Fn(Event)) {
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

/// Experimental: Watch a directory for changes asynchronously
pub fn watch_async(
    path: &Path,
    recursive: bool,
    events: &Vec<String>,
    f: impl Fn(Event) + Send + Sync + 'static,
    num_threads: usize,
) {
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

    let pool = ThreadPool::new(num_threads);

    let f = Arc::new(f);

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

                if kind_str == "all" || events.contains(&kind_str) {
                    let f = Arc::clone(&f);
                    pool.execute(move || {
                        f(event);
                    });
                }
            }

            Err(e) => {
                println!("watch error: {:?}", e);
            }
        }
    }
}
