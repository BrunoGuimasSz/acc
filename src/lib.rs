#[derive(Clone)]
struct Command {
    name: String,
    description: Option<String>,
    subcommand: Vec<Command>,
    arg: Option<Box<Arg>>,
}

#[derive(Clone)]
struct Arg {
    name: String,
    description: Option<String>,
    short: Option<char>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            description: None,
            subcommand: Vec::new(),
            arg: None,
        }
    }
    
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn subcommand(mut self, subcommand: Command) -> Self {
        self.subcommand.push(subcommand);
        self
    }

    pub fn arg(mut self, arg: Arg) -> Self {
        self.arg = Some(Box::new(arg));
        self
    }
}

impl Arg {
    pub fn new(name: &str) -> Self {
        Arg {
            name: name.to_string(),
            description: None,
            short: None,
        }
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_command() {
        let matches = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                .arg(
                    Arg::new("a")
                    .description("Show all files")
                    .short('a')
                )
                .arg(
                    Arg::new("l")
                    .description("Show long files")
                    .short('l')
                )
            );

        assert_eq!(matches.name, "linux");
        assert_eq!(matches.description, Some("Linux kernel".to_string()));
        assert_eq!(
            matches.subcommand[0].name,
            "ls"
            );
        assert_eq!(
            matches.subcommand[0].description,
            Some("List files and dirs".to_string())
            );
        
    }
}
