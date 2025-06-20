pub struct CommandParsed {
    pub command: String,
    pub subcommand: Option<String>,
    pub flag: Vec<String>,
    //value: Option<String>,
    //arg_vec: Vec<String>,
}
