use crate::{command::Command, parser::CommandParsed};

#[derive(Clone)]
pub struct App {
    commands: Vec<Command>
}

impl App {
    pub fn new() -> Self {
        App { commands: Vec::new() }
    }

    pub fn command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn parse(&self, tokens: Vec<String>) -> CommandParsed {
        let cmd_name = &tokens[1];

        if let Some(cmd) = self.commands.iter().find(|c| c.get_name() == cmd_name) {
            cmd.clone().parse(tokens)
        } else {
            panic!("Unknow command: {}", cmd_name)
        }
    }
}