//! # WatchCrab
//!
//! `watchcrab` is a command-line tool written in Rust that monitors directories for filesystem events such as file creation, modification, and deletion. It leverages the `notify` crate to efficiently watch directories and trigger specific actions when events occur.
//!
//! This tool is particularly useful for automating file processing tasks or generating logs based on filesystem changes. You can pass shell commands as arguments, which will be executed when the corresponding event is triggered, allowing for flexible automation workflows.
//!
//! In addition to the command-line tool, you can also integrate the `watchcrab` crate directly into your Rust project. This gives you finer control over how to handle them programmatically, making it a versatile option for more complex or customized file monitoring needs.

//Re-export the main functions for the crate
pub use self::watch::watch_async;
pub use self::watch::watch_sync;

pub mod util;
pub mod watch;
