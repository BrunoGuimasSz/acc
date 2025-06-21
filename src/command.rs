// TODO Diminuir a quantidade de .clone()
use crate::flag::Flag;
use crate::parser::CommandParsed;

#[derive(PartialEq, Clone)]
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
        let flags = get_flag(&self, tokens.clone());
        let value = get_value(tokens.clone(), &subcommand, flags.as_ref().unwrap());

        CommandParsed {
            command: command.unwrap(),
            subcommand: subcommand,
            flag: flags.unwrap(),
            value: value,
            arg_vec: tokens,
        }
    }
}

fn get_command_name(command: &Command, tokens: &Vec<String>) -> Result<String, String> {
    let cmd_name = tokens.get(1);
    match tokens.get(1) {
        Some(name) if name == command.get_name() => Ok(name.to_owned()),
        _ => Err(format!("Error: Command not found: {:?}", cmd_name)),
    }
}

fn get_subcommands(command: &Command, tokens: &Vec<String>) -> Option<String> {
    if tokens.len() < 2 {
        return None;
    }

    if command.subcommand.iter().any(|sub| sub.name == tokens[2]) {
        Some(tokens[2].to_owned())
    } else {
        None
    }
}

// FIXME Arrumar esse metodo
fn get_flag(command: &Command, tokens: Vec<String>) -> Result<Vec<String>, String> {
    Ok(vec![tokens[3].clone(), tokens[4].clone()])
}

fn get_value(
    tokens: Vec<String>,
    subcommand: &Option<String>,
    flags: &Vec<String>,
) -> Option<String> {
    let mut skip = if let Some(_sub) = subcommand { 3 } else { 2 };

    skip += flags.len();

    let value: String = tokens.into_iter().skip(skip).collect();

    if value.len() != 0 { Some(value) } else { None }
}
