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
