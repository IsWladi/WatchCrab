///Replace the '{path}' placeholder in a string with a given path
///
/// # Arguments
/// * `command` - Command to replace the placeholder in
/// * `path` - Path to replace the placeholder with
pub fn parse_command(command: &Vec<String>, path: &str) -> Vec<String> {
    let mut parsed_command = Vec::new();
    for arg in command.clone() {
        if arg.contains("{path}") {
            parsed_command.push(arg.replace("{path}", path))
        } else {
            parsed_command.push(arg);
        }
    }
    parsed_command
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_simple() {
        let command = vec!["echo".to_string(), "{path}".to_string()];
        let path = "/tmp";
        let expected = vec!["echo".to_string(), "/tmp".to_string()];
        assert_eq!(parse_command(&command, path), expected);
    }

    #[test]
    fn test_parse_command_compound() {
        let command = vec![
            "echo".to_string(),
            "/path/to/other/thing/{path}".to_string(),
        ];
        let path = "tmp";
        let expected = vec!["echo".to_string(), "/path/to/other/thing/tmp".to_string()];
        assert_eq!(parse_command(&command, path), expected);
    }
}
