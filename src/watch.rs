use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::{path::Path, sync::Arc};

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use threadpool::ThreadPool;

#[cfg(target_family = "unix")]
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

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
    #[allow(dead_code)]
    num_threads: usize, // is used in the constructor for initializing the thread pool
    pool: Option<ThreadPool>,
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
            pool: if num_threads > 1 {
                Some(ThreadPool::new(num_threads))
            } else {
                None
            },
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

        let is_windows = if cfg!(windows) { true } else { false };

        let term = Arc::new(AtomicBool::new(false));

        #[cfg(unix)]
        {
            let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
            let term_clone = Arc::clone(&term);
            thread::spawn(move || {
                for sig in signals.forever() {
                    if sig == SIGINT || sig == SIGTERM {
                        term_clone.store(true, Ordering::Relaxed);
                        break; // Break the signal loop
                    }
                }
            });
        }

        let mut cleaning_mode = false;

        loop {
            if !is_windows && term.load(Ordering::Relaxed) && !cleaning_mode {
                // Stop the watcher
                let _ = watcher.unwatch(self.path.canonicalize().unwrap().as_path());
                cleaning_mode = true;
                // Continue to process remaining events
            }

            if !is_windows && cleaning_mode {
                // Use try_recv to process remaining events
                match rx.try_recv() {
                    Ok(event_result) => {
                        process_event(event_result, &self.events, &self.f, &self.pool);
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
                        // No more events, break the loop
                        break;
                    }
                    Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                        // Channel disconnected, break the loop
                        break;
                    }
                }
            } else {
                match rx.recv_timeout(Duration::from_millis(100)) {
                    Ok(event_result) => {
                        process_event(event_result, &self.events, &self.f, &self.pool);
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Timeout occurred, check if termination flag is set
                        if !is_windows && term.load(Ordering::Relaxed) {
                            // Stop the watcher
                            let _ = watcher.unwatch(self.path.canonicalize().unwrap().as_path());
                            cleaning_mode = true;
                            continue;
                        }
                        // Else, continue the loop
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        // Channel disconnected, break the loop
                        break;
                    }
                }
            }
        }

        //wait for all threads to finish
        if let Some(pool) = &self.pool {
            pool.join();
        }

        Ok(())
    }
}

fn process_event(
    event_result: Result<Event, notify::Error>,
    events_filter: &Vec<String>,
    handler: &Arc<Box<dyn Fn(Event) + Send + Sync + 'static>>,
    pool: &Option<ThreadPool>,
) {
    match event_result {
        Ok(event) => {
            let kind_str = if events_filter == &["all"] {
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
                return;
            };

            let kind_str = String::from(kind_str);

            if kind_str == "all" || events_filter.contains(&kind_str) {
                if let Some(pool) = pool {
                    let f = Arc::clone(handler);
                    pool.execute(move || {
                        f(event.clone());
                    });
                } else {
                    handler(event)
                }
            }
        }
        Err(e) => {
            println!("Watch error: {:?}", e);
        }
    }
}
