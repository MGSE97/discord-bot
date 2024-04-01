use std::fs;

use poise::{command, serenity_prelude::Message};

use tracing::instrument;

use crate::{log_cmd, Context, Error};

/// Owl will magicaly turn this message into dataset.
#[instrument(level = "trace", skip(ctx), ret)]
#[command(context_menu_command = "dataset", owners_only)]
pub async fn dataset(
    ctx: Context<'_>,
    #[description = "Message"] message: Message,
) -> Result<(), Error> {
    // Prepare and log response
    log_cmd!(ctx::dataset(
        message_id = message.id.to_string(),
        channel_id = message.channel_id.to_string(),
        author_id = message.author.id.to_string(),
        author_name = message.author.name,
        author_nickname = message.author.global_name.clone().unwrap_or_default()
    ));

    fs::create_dir_all("datasets")?;
    fs::write(
        "datasets/easter.ds",
        message.content_safe(ctx.cache()).replace(". ", ","),
    )?;

    ctx.reply("Created new easter dataset").await?;
    Ok(())
}
