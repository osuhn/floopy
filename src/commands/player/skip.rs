use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::error_embed,
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Skips the current song.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "skip",
	guild_only,
	aliases("s"),
	category = "Player",
	member_cooldown = 5
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, false, |conn, ctx| async move {
		let driver = conn.lock().await;

		if driver.queue().is_empty() {
			ctx.send(poise::CreateReply::default().embed(
				error_embed(CreateEmbed::default()).description("There is no song to skip."),
			))
			.await?;
			return Ok(());
		}

		let _ = driver.queue().skip();

		ctx.send(poise::CreateReply::default().content("Skipped the current song."))
			.await?;

		return Ok(());
	})
	.await
}
