#[derive(Clone)]
struct Command {
    name: String,
    description: Option<String>,
    subcommand: Option<Box<Command>>
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            description: None,
            subcommand: None
        }
    }
    
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.subcommand = Some(Box::new(subcommand.clone()));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_command() {
        let args = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("touch")
                .description("Creates a file")
            );

        assert_eq!(args.name, "linux");
        assert_eq!(args.description, Some("Linux kernel".to_string()));
        assert_eq!(
            args.subcommand.as_ref().unwrap().name,
            "touch"
            );
        assert_eq!(
            args.subcommand.as_ref().unwrap().description,
            Some("Creates a file".to_string())
            );
    }
}
