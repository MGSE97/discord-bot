use base64::{engine::general_purpose::STANDARD, Engine as _};
use poise::{
    command,
    serenity_prelude::{json::json, Attachment, CreateEmbed, DiscordJsonError, User, USER_AGENT},
    CreateReply,
};
use reqwest::Client;

use crate::{Context, Error};

/// Owl will go to barber.
#[command(slash_command, owners_only)]
#[tracing::instrument(skip(ctx, avatar))]
pub async fn barber(
    ctx: Context<'_>,
    #[description = "Owl icon"] avatar: Attachment,
) -> Result<(), Error> {
    let channel_id = ctx.channel_id();

    let message_str = "🐱‍🏍 Going to barber ...".to_string();
    let reply = ctx.reply(&message_str).await?;

    ctx.http().broadcast_typing(channel_id).await?;
    ctx.defer_ephemeral().await?;

    let message = CreateReply::default()
        .content(format!("{message_str}\n🐱‍👤 Barber is working ... "))
        .embed(CreateEmbed::new().image(&avatar.url));

    reply.edit(ctx, message.clone()).await?;

    // We download and convert image to base64 (not a good approach, too much data trasfer)
    // Using CDN directly didn't worked (tested in PM)
    // ToDo: try to get CDN working
    let img = format!(
        "data:{mime};base64,{data}",
        mime = avatar
            .content_type
            .as_ref()
            .expect("Image content type is missing!"),
        data = STANDARD.encode(avatar.download().await?)
    );

    let token = ctx.http().token();
    let client = Client::new();
    let response = client
        .patch("https://discord.com/api/v10/users/@me")
        .header("Authorization", token)
        .header("User-Agent", USER_AGENT)
        .header("Content-Type", "application/json")
        .json(&json!({ "avatar": img }))
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => {
            let data = res.json::<User>().await?;
            reply
                .edit(
                    ctx,
                    message
                        .clone()
                        .content(format!(
                            "{msg}\n✅ Barber finished!",
                            msg = message.content.unwrap_or_default()
                        ))
                        .embed(
                            CreateEmbed::new().image(data.avatar_url().expect("No avatar, why?")),
                        ),
                )
                .await?;
            Ok(())
        }
        Ok(res) => {
            let err = res.json::<DiscordJsonError>().await?;
            reply
                .edit(
                    ctx,
                    message.clone().content(format!(
                        "{msg}\n💥 Barber failed!\n```{err:#?}```\n",
                        msg = message.content.unwrap_or_default()
                    )),
                )
                .await?;
            Err(err.message.into())
        }
        Err(err) => {
            reply
                .edit(
                    ctx,
                    message.clone().content(format!(
                        "{msg}\n💥 Barber totaly failed!\n```{err}```\n",
                        msg = message.content.unwrap_or_default()
                    )),
                )
                .await?;
            Err(err.into())
        }
    }
}
