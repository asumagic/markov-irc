use anyhow::*;
use irc::client::ClientStream;
use irc::client::prelude::*;
use tokio::stream::StreamExt;

pub struct Bot
{
    pub client: Client,
    pub stream: ClientStream,
}

impl Bot {
    pub async fn new(config: Config) -> Result<Bot> {
        let mut client = Client::from_config(config).await?;
        client.identify()?;

        let stream = client.stream()?;

        Ok(Bot { client, stream })
    }

    pub async fn next_message(&mut self) -> Result<Option<Message>> {
        Ok(self.stream.next().await.transpose()?)
    }

    pub fn send_error(&self, target: &str, message: Option<&Message>, error: Error) -> Result<()> {
        let sender = &self.client.sender();

        match message {
            Some(message) => {
                sender.send_privmsg(&target, std::format!("{}: error: {}", message.source_nickname().ok_or_else(|| anyhow!("wat"))?, error.root_cause()))?;
            }
            None => {
                sender.send_privmsg(&target, std::format!("general error: {}", error.root_cause()))?;
            }
        }

        Ok(())
    }
}
