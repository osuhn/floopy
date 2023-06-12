use crate::{shared::get_conn, FloopyContext, FloopyError};

/// Skips the current song.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "skip",
	guild_only,
	aliases("s")
)]
pub async fn command(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	ctx.defer().await?;

	let conn = get_conn(&ctx).await?;
	let driver = conn.lock().await;

	if driver.queue().is_empty() {
		ctx.send(|r| r.content("There is no song to skip.")).await?;
		return Ok(());
	}

	let _ = driver.queue().skip();

	ctx.send(|r| r.content("Skipped the current song.")).await?;

	return Ok(());
}
