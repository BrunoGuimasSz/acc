use std::env;

struct Command {
    name: String,
    description: Option<String>,
    subcommand: Vec<Command>,
    flag: Vec<Flag>,
}

struct Flag {
    name: String,
    description: Option<String>,
    short: Option<char>,
}

struct CommandParsed {
    command: Option<String>,
    subcommand: Option<String>,
    flag: Option<String>,
    value: Option<String>,
    arg_vec: Vec<String>,
}

impl Command {
    pub fn new(name: &str) -> Self {
        Command {
            name: name.to_string(),
            description: None,
            subcommand: Vec::new(),
            flag: Vec::new(),
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

    pub fn flag(mut self, flag: Flag) -> Self {
        self.flag.push(flag);
        self
    }

    pub fn get_result(self) -> CommandParsed {
        let tokens: Vec<String> = env::args().collect();

        let command = get_command_name(self, tokens);

        CommandParsed {
            command: String,
            subcommand: Option<String>,
            flag: Option<String>,
            value: Option<String>,
            arg_vec: Option<String>,
        }
    }
}

impl Flag {
    pub fn new(name: &str) -> Self {
        Flag {
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

fn get_command_name(command: Command, tokens: Vec<String>) -> String {
    if tokens[0] == command.name {
        tokens[0].clone()
    } else {
        panic!("Error")
    }
}

fn get_subcommand_name(subcommand: Command, tokens: Vec<String>) -> Option<String> {
    for token in tokens {
        if token == subcommand.name {
            Some(token)
        } else {
            None
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_command() {
        let tokens = Command::new("linux")
            .description("Linux kernel")
            .subcommand(
                Command::new("ls")
                .description("List files and dirs")
                .flag(
                    Flag::new("a")
                    .description("Show all files")
                    .short('a')
                )
                .flag(
                    Flag::new("l")
                    .description("Show long files")
                    .short('l')
                )
            );

        assert_eq!(tokens.name, "linux");
        assert_eq!(tokens.description, Some("Linux kernel".to_string()));
        assert_eq!(
            tokens.subcommand[0].name,
            "ls"
            );
        assert_eq!(
            tokens.subcommand[0].description,
            Some("List files and dirs".to_string())
            );
        assert_eq!(
            tokens.subcommand[0].flag[0].name,
            "a"
        );
        assert_eq!(
            tokens.subcommand[0].flag[0].description,
            Some("Show all files".to_string())
        );
    }
}
