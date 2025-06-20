#[derive(PartialEq, Clone)]
pub struct Flag {
    name: String,
    description: Option<String>,
    short: Option<char>,
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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_short(&self) -> Option<char> {
        self.short
    }

} 

