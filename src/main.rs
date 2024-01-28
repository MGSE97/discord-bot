use std::env;

use dotenv::dotenv;
use serenity::{
    all::Ready,
    async_trait,
    framework::standard::{macros::group, Configuration, StandardFramework},
    prelude::*,
};
use tracing::{error, info};
use tracing_subscriber::fmt;

mod commands;
mod handlers;

use crate::commands::hoo::*;

#[group]
#[commands(hoo)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Connected to: {:#?}", ready.guilds);
        for guild in ready.guilds {
            if !guild.unavailable {
                let new_commands = guild
                    .id
                    .set_commands(&ctx.http, vec![commands::hoo::register()])
                    .await;
                info!("Set guild({}) slash commands: {new_commands:#?}", guild.id);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    fmt().init();

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(
        Configuration::new() // Configure bot
            .prefixes(vec!["/", "!", ":owl:", "🦉"])
            .case_insensitivity(true),
    );

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is missing in environment!");
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start().await {
        Err(err) => error!("An error occurred while running the client: {err:?}"),
        Ok(_) => info!("Started client"),
    }
}
