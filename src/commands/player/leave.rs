use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::error_embed,
	shared::{enter_vc, leave_channel},
	structs::{CommandResult, Context},
};

/// Leaves the current voice channel.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "leave",
	guild_only,
	aliases("stop"),
	category = "Player",
	member_cooldown = 5
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, false, |conn, ctx| async move {
		if !conn.lock().await.queue().is_empty() {
			let _ = conn.lock().await.queue().stop();
		}

		match leave_channel(&ctx).await {
			Ok(_) => {
				ctx.send(poise::CreateReply::default().content("I left the voice channel"))
					.await?;

				return Ok(());
			}

			Err(_) => {
				ctx.send(
					poise::CreateReply::default()
						.embed(error_embed(CreateEmbed::default().description(
							"An error occurred while trying to leave channel.",
						))),
				)
				.await?;

				return Ok(());
			}
		}
	})
	.await
}
