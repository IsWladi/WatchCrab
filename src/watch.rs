use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};

use std::sync::mpsc::channel;
use std::{path::Path, sync::Arc};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use threadpool::ThreadPool;

/// Watch a directory for changes synchronously or asynchronously depending on the number of threads
///
/// # Arguments
/// * `path` - Path to the directory to watch
/// * `recursive` - Watch directories recursively
/// * `events` - Events to watch for
///     e.g. ["all"] or ["access", "create", "modify", "remove"]
/// * `f` - Function to handle the events, it receives an `Event` object
/// * `num_threads` - Number of threads to use, if 1 it will run synchronously, if greater than 1 it will run asynchronously
///
/// # Examples
///
/// **Print all filesystem events in the current directory**
///
/// ```no_run
/// use std::path::Path;
/// use notify::Event;
/// use std::sync::Arc;
/// use watchcrab::watch::Watch;
///
///
/// let path = Path::new("./"); // Watch the current directory, you can change this to any path
/// let recursive = false; // Watch only the top level directory, you can change this to true
/// let events = vec!["all".to_string()]; // Watch all events, you can change this to ["access", "create", "modify", "remove"] or any combination of these, at least one is required
/// let f = Arc::new(Box::new(move |event: Event| {
///    println!("{:?}", event); // Print the event, you can replace this with your own logic
/// }) as Box<dyn Fn(Event) + Send + Sync + 'static>);
///
/// Watch::new(&path, recursive, &events, f, 1).start();
/// ```

pub struct Watch<'a> {
    path: &'a Path,
    recursive: bool,
    events: &'a Vec<String>,
    f: Arc<Box<dyn Fn(Event) + Send + Sync + 'static>>,
    num_threads: usize,
}

impl Drop for Watch<'_> {
    fn drop(&mut self) {
        println!("Dropping Watch");
    }
}

impl<'a> Watch<'a> {
    pub fn new(
        path: &'a Path,
        recursive: bool,
        events: &'a Vec<String>,
        f: Arc<Box<dyn Fn(Event) + Send + Sync + 'static>>,
        num_threads: usize,
    ) -> Watch<'a> {
        Watch {
            path,
            recursive,
            events,
            f,
            num_threads,
        }
    }

    pub fn start(&self) -> Result<(), Error> {
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        let recursive_mode = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher
            .watch(self.path.canonicalize().unwrap().as_path(), recursive_mode)
            .unwrap();

        let pool: Option<ThreadPool> = if self.num_threads > 1 {
            Some(ThreadPool::new(self.num_threads))
        } else {
            None
        };

        let term = Arc::new(AtomicBool::new(false));
        signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&term))?;

        for event in rx {
            if term.load(Ordering::Relaxed) {
                println!("Exiting watcher loop");
                return Ok(());
            }
            match event {
                Ok(event) => {
                    let kind_str = if self.events == &["all"] {
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

                    if kind_str == "all" || self.events.contains(&kind_str) {
                        if let Some(pool) = &pool {
                            let f = Arc::clone(&self.f);
                            pool.execute(move || {
                                f(event);
                            });
                        } else {
                            (self.f)(event)
                        }
                    }
                }

                Err(e) => {
                    println!("watch error: {:?}", e);
                }
            }
        }
        Ok(())
    }
}
