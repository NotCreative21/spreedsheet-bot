use std::io::Result;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use optimal::Storage;

struct Handler {
    prefix: String,
    access: Vec<u64>,
    db_path: String
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content[..1] != self.prefix.to_owned() { return; }

        let message = &msg.content[1..];

        let response: Option<String> = match message {
            "ping" => {
                Storage::push().await;
                None
            },
            _ => return,
        };

        if response.is_none() { return; }

        if let Err(why) = msg.channel_id.say(&ctx.http, response.unwrap()).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = optimal::Config::load()?;

    let mut client =
        Client::builder(&config.token).event_handler(
            Handler {
                prefix: config.prefix.to_owned(),
                access: config.access,
                db_path: config.db_path.to_owned()
            }
            ).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    Ok(())
}
