use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::base_embed,
	metadata::{format_metadata, AuxMetadataKey},
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Removes a song from the queue.
#[poise::command(
	category = "Player",
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "remove",
	aliases("r", "rm", "delete", "del"),
	guild_only,
	member_cooldown = 5
)]
pub async fn command(
	ctx: Context<'_>,
	#[description = "The index of the song to remove."] index: usize,
) -> CommandResult {
	enter_vc(ctx, false, |handler, ctx| async move {
		if index == 0 {
			ctx.send(poise::CreateReply::default().embed(
				base_embed(CreateEmbed::default()).description("Cannot remove the current song"),
			))
			.await?;
			return Ok(());
		}

		let handler = handler.lock().await;

		let result = handler.queue().modify_queue(|x| {
			if let Some(track) = x.remove(index) {
				if let Err(e) = track.stop() {
					Err(format!("Failed to stop track: {:?}", e))
				} else {
					Ok(track)
				}
			} else {
				Err(format!("No track at index {index}"))
			}
		});

		drop(handler);

		match result {
			Ok(track) => {
				let map = track.typemap().read().await;
				let metadata = map.get::<AuxMetadataKey>().unwrap();
				ctx.send(
					poise::CreateReply::default().embed(
						CreateEmbed::default()
							.description(format!("Removed: {}", format_metadata(metadata))),
					),
				)
				.await?;
			}
			Err(e) => {
				ctx.say(&e).await?;
			}
		}

		Ok(())
	})
	.await
}
