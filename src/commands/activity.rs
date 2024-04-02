use poise::{command, serenity_prelude::ActivityData};
use tracing::instrument;

use crate::{log_cmd, Context, Error};

/// Give Owl some task.
#[instrument(level = "trace", skip(ctx), ret)]
#[command(prefix_command, slash_command, owners_only)]
pub async fn activity(
    ctx: Context<'_>,
    #[description = "New activity"] activity: Option<String>,
) -> Result<(), Error> {
    let activity_fmt = activity
        .clone()
        .map(|a| format!("{a:?}"))
        .unwrap_or_else(|| "None".to_string());

    log_cmd!(ctx::activity(activity = activity_fmt));

    ctx.serenity_context()
        .set_activity(activity.clone().map(ActivityData::custom));

    let response = format!("Set activity to {activity_fmt}.");
    ctx.reply(&response).await?;

    Ok(())
}
