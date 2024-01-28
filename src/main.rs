use std::env;

use dotenv::dotenv;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, Configuration, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(hoo)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("🦉")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is missing in environment!");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn hoo(ctx: &Context, msg: &Message) -> CommandResult {
    let number = msg
        .content
        .to_string()
        .split_ascii_whitespace()
        .find_map(|x| x.parse::<u64>().ok());

    let mut response = "🦉 Hoo!".to_string();

    if let Some(number) = number {
        response = format!(
            "{response}\nZa {number} si {}",
            match number {
                p if p < 50 => "ani ptáčka nekoupíš.".to_string(),
                p if p < 15_000 => format!("koupíš {} ptáčků.", number / 50),
                _ => format!("koupíš {} sovy.", number / 15_000),
            }
        );
    }

    msg.reply(ctx, &response).await?;

    Ok(())
}
