use std::collections::HashMap;

use anyhow::*;
use irc::client::prelude::*;

use ircbot::*;
use markov::*;
use markovtrainer::*;

mod markovtrainer;
mod msgprocessor;
mod logparse;
mod ircbot;
mod markov;

struct CommandImpersonate<'a>
{
    impersonated: &'a str,
    hook: &'a str,
}

impl<'a> CommandImpersonate<'a>
{
    fn from(words: &'a [&str]) -> Result<Self>
    {
        let bad_syntax = || anyhow!("expected args: [user] [hook word]");

        Ok(CommandImpersonate {
            impersonated: words.get(0).ok_or_else(bad_syntax)?,
            hook: words.get(1).ok_or_else(bad_syntax)?,
        })
    }

    fn handle(sender: &Sender, markovs: &HashMap<String, Markov>, target: &str, words: &'a [&str]) -> Result<()> {
        let cmd = Self::from(&words)?;

        let markov = markovs.get(cmd.impersonated).ok_or_else(|| anyhow!(r#"user "{}" not found"#, &cmd.impersonated))?;

        let random_chain = markov.random_chain(cmd.hook)?;

        sender.send_privmsg(&target, random_chain.join(" "));

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let markovs = markov_from_logs(&["/home/sdelang/logs/mibbit_#cbna.log",
        "/home/sdelang/logs/freenode_#cbna.log"]);

    let mut bot = Bot::new(irc::client::prelude::Config {
        nickname: Some("asubot".to_owned()),
        server: Some("chat.freenode.net".to_owned()),
        channels: vec!["#cbna-bot-spam".to_owned(), "#cbna".to_owned()],
        ..Default::default()
    }).await?;

    let client = &bot.client;
    let sender = &client.sender();

    while let Some(message) = bot.next_message().await? {
        print!("{}", message);

        match message.command {
            Command::PRIVMSG(ref target, ref msg) => {
                let mut splits = msg.split_whitespace();

                splits.next().map(|command| {
                    let args: Vec<&str> = splits.collect();
                    match command {
                        "~impersonate" => {
                            match CommandImpersonate::handle(&sender, &markovs, &target,&args) {
                                Ok(cmd) => {}
                                Err(err) => { bot.send_error(&target, Some(&message), err); }
                            }
                        }
                        _ => {}
                    }
                });
            }
            _ => {}
        }
    }

    Ok(())
}
