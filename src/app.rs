use crate::{command::Command, parser::CommandParsed};

#[derive(Clone)]
pub struct App {
    commands: Vec<Command>,
}

impl App {
    pub fn new() -> Self {
        App {
            commands: Vec::new(),
        }
    }

    pub fn command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn parse(&self, tokens: Vec<String>) -> Result<CommandParsed, String> {
        if tokens.len() < 2 {
            return Err(format!("Error: Argument too short"))
        }

        let cmd_name = &tokens[1];

        if let Some(cmd) = self.commands.iter().find(|c| c.get_name() == cmd_name) {
            Ok(cmd.clone().parse(tokens))
        } else {
            Err(format!("Error: Command not found: {}", cmd_name))
        }
    }
}
