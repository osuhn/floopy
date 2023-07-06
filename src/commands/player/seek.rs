use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::base_embed,
	error::{NotInVoiceChannelError, SeekTimeTooLargeError},
	metadata::AuxMetadataKey,
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Seeks to a certain time in the current track.
#[poise::command(
	category = "Player",
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "seek",
	aliases("sk"),
	guild_only,
	member_cooldown = 5
)]
pub async fn command(
	ctx: Context<'_>,
	#[description = "The time to seek to in seconds."]
	#[min = 1]
	#[rest]
	time: humantime::Duration,
) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, true, |conn, ctx| async move {
		let lock = conn.lock().await;
		let current = lock.queue().current().ok_or(NotInVoiceChannelError)?;

		let map = current.typemap().read().await;
		let metadata = map.get::<AuxMetadataKey>().unwrap();

		if let Some(track_time) = metadata.duration {
			if track_time.as_secs().le(&time.as_secs()) {
				Err(SeekTimeTooLargeError(
					humantime::Duration::from(track_time).to_string(),
				))?
			}
		}

		current.seek_async(time.into()).await?;

		ctx.send(
			poise::CreateReply::default().embed(
				base_embed(CreateEmbed::default()).description(format!("Seeked to {}.", time)),
			),
		)
		.await?;

		Ok(())
	})
	.await
}
