use std::string::String;

#[derive(Debug, RustcEncodable)]
pub struct ChatLine {
    tick: u32,
    name: String,
    message: String,
}

impl ChatLine {
    pub fn new() -> ChatLine {
        ChatLine {
            tick: 0,
            name: String::new(),
            message: String::new(),
        }
    }

    pub fn with_data<S>(tick: u32, name: S, message: S) -> ChatLine where S: Into<String> {
        ChatLine {
            tick: tick,
            name: name.into(),
            message: message.into(),
        }
    }

    pub fn display(&self) {
        println!("{} - {}: {}", self.tick, self.name, self.message);
    }
}