use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::{base_embed, error_embed},
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Display or set the volume of the player.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "volume",
	guild_only,
	aliases("vol"),
	category = "Player",
	member_cooldown = 5
)]
pub async fn command(
	ctx: Context<'_>,
	#[description = "The volume to set the player to. Must be between 10 and 100."]
	#[min = 10]
	#[max = 100]
	volume: Option<f32>,
) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, false, |conn, ctx| async move {
		let track = match conn.lock().await.queue().current() {
			Some(track) => track,
			None => {
				ctx.send(
					poise::CreateReply::default().embed(
						error_embed(CreateEmbed::default())
							.description("No track is currently playing!"),
					),
				)
				.await?;

				return Ok(());
			}
		};

		if let Some(vol) = volume {
			let _ = track.set_volume(vol / 100.0);

			ctx.send(poise::CreateReply::default().embed(
				base_embed(CreateEmbed::default()).description(format!("Volume set to {vol}%.")),
			))
			.await?;

			return Ok(());
		}

		if let Ok(track_info) = track.get_info().await {
			let vol = track_info.volume * 100.0;
			ctx.send(
				poise::CreateReply::default().embed(
					base_embed(CreateEmbed::default())
						.description(format!("Current volume is {vol}%",)),
				),
			)
			.await?;

			return Ok(());
		}

		ctx.send(
			poise::CreateReply::default().embed(
				error_embed(CreateEmbed::default()).description("Failed to get track info."),
			),
		)
		.await?;

		Ok(())
	})
	.await
}
