use humantime::format_duration;

use serenity_feature_only::builder::CreateEmbed;
use songbird::input::{Compose, Input, YoutubeDl};

use crate::{
	commands::base_embed,
	metadata::AuxMetadataKey,
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Plays a song.
#[poise::command(
	category = "Player",
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "play",
	aliases("p", "pl"),
	guild_only,
	member_cooldown = 5
)]
pub async fn command(
	ctx: Context<'_>,
	#[description = "The song to play"]
	#[rest]
	query: String,
) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, false, |conn, ctx| async move {
		let reqwest = ctx.data().reqwest.clone();
		let mut source = YoutubeDl::new(reqwest, query);

		let channel_id = ctx.channel_id();
		let metadata = source.aux_metadata().await?;
		let handle = conn.lock().await.enqueue_input(Input::from(source)).await;

		// To provent the bot from earaping people
		let _ = handle.set_volume(0.5);

		let mut typemap = handle.typemap().write().await;
		typemap.insert::<AuxMetadataKey>(metadata.clone());

		ctx.send({
			let builder = poise::CreateReply::default();

			builder.embed(
				base_embed(CreateEmbed::default())
					.title(format!("Queueing audio in <#{channel_id}>"))
					.field(
						"Title",
						if metadata.title.is_some() {
							metadata.title.unwrap()
						} else {
							metadata.track.unwrap()
						},
						false,
					)
					.field(
						"Duration",
						if metadata.duration.is_some() {
							format_duration(metadata.duration.unwrap()).to_string()
						} else {
							"∞".to_string()
						},
						true,
					)
					.field(
						"Source",
						format!(
							"[Open original]({})",
							metadata.source_url.unwrap_or(
								"https://www.youtube.com/watch?v=dQw4w9WgXcQ".to_string()
							)
						),
						true,
					)
					.thumbnail(metadata.thumbnail.unwrap()),
			)
		})
		.await?;

		return Ok(());
	})
	.await
}
