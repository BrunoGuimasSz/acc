use crate::flag::Flag;
use crate::parser::CommandParsed;

#[derive(PartialEq)]
pub struct Command {
    name: String,
    description: Option<String>,
    subcommand: Vec<Command>,
    flag: Vec<Flag>,
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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_subcommands(&self) -> &[Command] {
        &self.subcommand
    }

    pub fn get_flags(&self) -> &[Flag] {
        &self.flag
    }

    pub fn parse(self, tokens: Vec<String>) -> CommandParsed {
        let command = get_command_name(&self, &tokens);
        let subcommand = get_subcommands(&self, &tokens);
        let flag = get_flag(tokens);

        CommandParsed {
            command: command,
            subcommand: subcommand,
            flag: flag,
        }
    }
}

fn get_command_name(command: &Command, tokens: &Vec<String>) -> String {
    if tokens[1] == command.name {
        tokens[1].to_owned()
    } else {
        panic!("Error")
    }
}

fn get_subcommands(command: &Command, tokens: &Vec<String>) -> Option<String> {
    if tokens.len() < 2 {
        return None
    }

    if command.subcommand.iter().any(|sub| &sub.name == &tokens[2]) {
        Some(tokens[2].clone())
    } else {
        None
    }
}

fn get_flag(tokens: Vec<String>) -> Vec<String> {
    tokens.into_iter().filter(|token| token.starts_with('-')).collect()
}
