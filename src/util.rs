use std::io::prelude::*;

#[cfg(target_family = "unix")]
use std::os::unix::process::CommandExt;

use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::{fs::OpenOptions, path::PathBuf};

#[cfg(target_family = "windows")]
use std::os::windows::process::CommandExt;

//https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/Threading/constant.CREATE_NO_WINDOW.html
#[cfg(target_family = "windows")]
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

lazy_static::lazy_static! {
    static ref LOG_FILE_MUTEX: Mutex<()> = Mutex::new(());
}

///Replace the '{path}' and '{kind}' placeholders in a command with the given path and kind
///
/// # Arguments
/// * `command` - Command to replace the placeholder in
/// * `path` - Path to replace the placeholder with
pub fn parse_command(command: &Vec<String>, path: &str, kind: &str) -> Vec<String> {
    let mut parsed_command = Vec::new();
    for arg in command.clone() {
        if arg.contains("{path}") || arg.contains("{kind}") {
            let tmp_arg = arg.replace("{path}", path);
            let tmp_arg = tmp_arg.replace("{kind}", kind);
            parsed_command.push(tmp_arg);
        } else {
            parsed_command.push(arg);
        }
    }
    parsed_command
}

///Write the output to a log file thread-safely
///
/// # Arguments
/// * `output_file_path` - Path to the log file
/// * `output` - Output to write to the log file
///
/// # Panics
/// Panics if the log file can't be opened
///
/// # Errors
/// Errors if the output can't be written to the log file
pub fn write_to_log_file_async(output_file_path: &PathBuf, output: &str) {
    let _lock = LOG_FILE_MUTEX.lock().unwrap(); // To make sure only one thread writes to the log file at a time
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file_path)
        .expect("Unable to open log file");
    if let Err(e) = writeln!(file, "{}", output) {
        eprintln!("Couldn't write to log file: {}", e);
    }
}

///Write the output to a log file
///
/// # Arguments
/// * `output_file_path` - Path to the log file
/// * `output` - Output to write to the log file
///
/// # Panics
/// Panics if the log file can't be opened
///
/// # Errors
/// Errors if the output can't be written to the log file
pub fn write_to_log_file(output_file_path: &PathBuf, output: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(output_file_path)
        .expect("Unable to open log file");
    if let Err(e) = writeln!(file, "{}", output) {
        eprintln!("Couldn't write to log file: {}", e);
    }
}

///Execute a command on Unix disabling the termination signal for the child process
#[cfg(target_family = "unix")]
pub fn command_exec_unix(sh_cmd_split: &Vec<String>, args_str: String) -> Child {
    unsafe {
        Command::new(&sh_cmd_split[0])
            .arg(&sh_cmd_split[1])
            .arg(args_str)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }

                libc::signal(libc::SIGINT, libc::SIG_IGN);

                Ok(())
            })
            .spawn()
            .expect("failed to execute command")
    }
}

///Execute a command on Windows
#[cfg(target_family = "windows")]
pub fn command_exec_windows(sh_cmd_split: &Vec<String>, args_str: String) -> Child {
    Command::new(&sh_cmd_split[0])
        .arg(&sh_cmd_split[1])
        .arg(args_str)
        .creation_flags(CREATE_NO_WINDOW.0)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute command")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_simple() {
        let command = vec![
            "echo".to_string(),
            "Path:".to_string(),
            "{path}".to_string(),
            "Kind:".to_string(),
            "{kind}".to_string(),
        ];
        let path = "/tmp";
        let kind = "Create";
        let expected = vec![
            "echo".to_string(),
            "Path:".to_string(),
            "/tmp".to_string(),
            "Kind:".to_string(),
            "Create".to_string(),
        ];
        assert_eq!(parse_command(&command, path, kind), expected);
    }

    #[test]
    fn test_parse_command_compound() {
        let command = vec![
            "echo".to_string(),
            "/path/to/other/thing/{path}".to_string(),
        ];
        let path = "tmp";
        let kind = "Create";
        let expected = vec!["echo".to_string(), "/path/to/other/thing/tmp".to_string()];
        assert_eq!(parse_command(&command, path, kind), expected);
    }
}
