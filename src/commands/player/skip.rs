use crate::{shared::join_channel, FloopyContext, FloopyError};

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

	let (_guild_id, _channel_id, conn, _manager) = join_channel(&ctx).await?;

	let _ = conn.lock().await.queue().skip();

	ctx.send(|r| r.content("Skipped the current song.")).await?;

	return Ok(());
}
