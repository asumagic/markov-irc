use crate::logparse::LogMessage;

pub struct ProcessedMessage {
    pub user: String,
    pub words: Vec<String>,
}

impl ProcessedMessage
{
    pub fn from(msg: LogMessage) -> ProcessedMessage {
        ProcessedMessage {
            user: msg
                .user
                .to_ascii_lowercase()
                .chars()
                .take_while(|c| !r#"[({"#.contains(*c))
                .collect::<String>(),

            words: msg
                .msg
                .split_whitespace()
                .filter(|word| !word.contains("://"))
                .map(|word| {
                    word.chars()
                        .filter(|c| c.is_alphanumeric() || r#"'"-_()"#.contains(*c))
                        .collect::<String>()
                        .to_lowercase()
                })
                .collect(),
        }
    }
}