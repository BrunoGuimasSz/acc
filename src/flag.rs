#[derive(PartialEq, Clone)]
pub struct Flag {
    long: String,
    description: Option<String>,
    short: Option<char>,
}

impl Flag {
    pub fn new(long: &str) -> Self {
        Flag {
            long: long.to_string(),
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

    pub fn get_long(&self) -> &str {
        self.long.as_str()
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn get_short(&self) -> Option<char> {
        self.short
    }
}
