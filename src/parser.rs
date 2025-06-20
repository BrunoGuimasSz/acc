pub struct CommandParsed {
    pub command: String,
    pub subcommand: Option<String>,
    pub flag: Vec<String>,
    pub value: Option<String>,
    pub arg_vec: Vec<String>,
}
