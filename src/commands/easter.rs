use itertools::Itertools;
use poise::command;
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use tokio::fs;
use tracing::instrument;

use crate::{log_cmd, Context, Error};

/// Owls easter moments.
#[instrument(level = "trace", skip(ctx), ret)]
#[command(prefix_command, slash_command)]
pub async fn easter(ctx: Context<'_>) -> Result<(), Error> {
    let dataset = fs::read_to_string("datasets/easter.ds").await?;
    let items = dataset
        .split('\n')
        .filter_map(|r| {
            let mut row = r.split(',');
            let id = row.next();
            let url = row.next();
            match (id, url) {
                (Some(id), Some(url)) => Some((id, url)),
                _ => None,
            }
        })
        .collect_vec();

    let mut rng = StdRng::seed_from_u64(ctx.id());
    let Some((id, url)) = items.choose(&mut rng) else {
        let err = "Failed to find resource :(";
        ctx.reply(err).await?;
        log_cmd!(
            ctx::easter() => err
        );
        Err(err)?
    };

    let response = format!(
        "🦉 Owl easter video {id} / {count} [🔗]({url})",
        count = items.len()
    );
    log_cmd!(
        ctx::easter() => response
    );

    ctx.reply(response).await?;

    Ok(())
}
