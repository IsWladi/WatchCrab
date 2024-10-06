use std::path::Path;

use clap::Parser;
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

    watch_sync(&path, args.recursive, &args.events);
}
