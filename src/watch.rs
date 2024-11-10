use std::io::Error;

use std::{path::Path, sync::Arc};

use crossbeam_channel::select;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use threadpool::ThreadPool;

#[cfg(target_family = "unix")]
use signal_hook::{
    consts::{SIGINT, SIGTERM},
    iterator::Signals,
};

use crossbeam_channel::unbounded;
use std::thread;

#[cfg(target_family = "windows")]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(target_family = "windows")]
use std::time::Duration;
#[cfg(target_family = "windows")]
use windows::Win32::Foundation::BOOL;
#[cfg(target_family = "windows")]
use windows::Win32::System::Console::{SetConsoleCtrlHandler, CTRL_CLOSE_EVENT, CTRL_C_EVENT};

// Global flag for Windows signal handling
#[cfg(target_family = "windows")]
static SHOULD_STOP: AtomicBool = AtomicBool::new(false);

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
        let (tx, rx) = unbounded();

        let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

        let recursive_mode = if self.recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };

        watcher
            .watch(self.path.canonicalize().unwrap().as_path(), recursive_mode)
            .unwrap();

        // Signal handling for graceful shutdown
        #[cfg(unix)]
        let signal_rx = {
            let (signal_tx, signal_rx) = unbounded();
            let mut signals = Signals::new(&[SIGINT, SIGTERM])?;
            thread::spawn(move || {
                for sig in signals.forever() {
                    if sig == SIGINT || sig == SIGTERM {
                        // Send signal to the main thread to stop the watcher
                        let _ = signal_tx.send(());
                        break;
                    }
                }
            });
            signal_rx
        };

        #[cfg(target_family = "windows")]
        let signal_rx = {
            let (signal_tx, signal_rx) = unbounded();
            unsafe {
                SetConsoleCtrlHandler(Some(console_handler), BOOL(1))
                    .expect("Failed to set control handler");
            }
            thread::spawn(move || {
                while !SHOULD_STOP.load(Ordering::SeqCst) {
                    thread::sleep(Duration::from_millis(100));
                }
                let _ = signal_tx.send(()); // Notify main loop to stop
            });
            signal_rx
        };

        loop {
            select! {
            recv(rx) -> event_result => {
                match event_result {
                    Ok(event_result) => {
                        process_event(event_result, &self.events, &self.f, &self.pool);
                    }
                    Err(_) => break, // Closed channel, exit the loop
                }
            }
                recv(signal_rx) -> _ => {
                    println!("Termination signal received. Stopping the watcher... Waiting for ongoing tasks to complete...");
                    let _ = watcher.unwatch(self.path.canonicalize().unwrap().as_path());
                    // Process pending events
                    while let Ok(event_result) = rx.try_recv() {
                        process_event(event_result, &self.events, &self.f, &self.pool);
                    }
                    break;
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

// Windows-specific console handler
#[cfg(target_family = "windows")]
unsafe extern "system" fn console_handler(ctrl_type: u32) -> BOOL {
    match ctrl_type {
        CTRL_C_EVENT | CTRL_CLOSE_EVENT => {
            SHOULD_STOP.store(true, Ordering::SeqCst);
            BOOL(1) // Return TRUE to indicate the event was handled
        }
        _ => BOOL(0),
    }
}
