use std::env;

use commands::{barber::*, hoo::*};
use dotenv::dotenv;
use poise::{
    serenity_prelude as serenity, Framework, FrameworkOptions, Prefix, PrefixFrameworkOptions,
};
use tracing::{error, info};
use tracing_subscriber::fmt;

mod commands;
#[macro_use]
pub(crate) mod logs;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    fmt().init();
    info!("RUST_LOG: {:?}", env::var("RUST_LOG"));

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hoo(), barber()],
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("/".to_string()),
                additional_prefixes: ["!", ":owl:", "🦉"]
                    .iter()
                    .map(|p| Prefix::Literal(p))
                    .collect(),
                case_insensitive_commands: true,
                ignore_bots: true,
                execute_self_messages: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN is missing in environment!");
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::GUILD_MESSAGES;

    let mut client = serenity::Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start_autosharded().await {
        Err(err) => error!("An error occurred while running the client: {err:?}"),
        Ok(_) => info!("Started client"),
    }
}
