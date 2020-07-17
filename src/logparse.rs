use lazy_static::*;
use regex::Regex;

// TODO: user and msg should not be owned ideally, as we may discard it early anyway
pub struct LogMessage {
    pub user: String,
    pub msg: String,
}

impl LogMessage {
    pub fn from_konversation(line: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"(?x)
                    \[(?P<date>[^\]]+)\]
                    \s*\[(?P<hour>[^\]]+)\]
                    \s*<(?P<user>[^>]+)>
                    \s*(?P<msg>.*)$
                "#
            )
            .unwrap();
        }

        if let Some(caps) = RE.captures(line) {
            Some(LogMessage {
                user: caps["user"].to_owned(),
                msg: caps["msg"].to_owned(),
            })
        } else {
            None
        }
    }
}
