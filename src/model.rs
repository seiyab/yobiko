use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub cwd: PathBuf,
    pub command: String,
    pub args: Vec<String>,
    pub content: Option<String>,
}

impl Task {
    pub fn command_line(&self) -> String {
        std::iter::once(self.command.as_str())
            .chain(self.args.iter().map(String::as_str))
            .map(shell_quote)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn with_shell_command(mut self, command: String) -> Self {
        self.command = "sh".into();
        self.args = vec!["-c".into(), command];
        self
    }
}

fn shell_quote(value: &str) -> String {
    if !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_@%+=:,./-".contains(&byte))
    {
        return value.into();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(command: &str, args: &[&str]) -> Task {
        Task {
            name: "example".into(),
            cwd: PathBuf::new(),
            command: command.into(),
            args: args.iter().map(|arg| (*arg).into()).collect(),
            content: None,
        }
    }

    #[test]
    fn command_line_quotes_shell_metacharacters() {
        assert_eq!(
            task("yarn", &["run", "test: unit", "it's-safe"]).command_line(),
            "yarn run 'test: unit' 'it'\\''s-safe'"
        );
    }

    #[test]
    fn shell_command_is_displayed_as_it_is_executed() {
        assert_eq!(
            task("yarn", &["run", "test"])
                .with_shell_command("yarn run test --runInBand".into())
                .command_line(),
            "sh -c 'yarn run test --runInBand'"
        );
    }
}

#[derive(Clone, Debug)]
pub struct Workspace {
    pub dir: PathBuf,
    pub provider: &'static str,
    pub tasks: Vec<Task>,
}
