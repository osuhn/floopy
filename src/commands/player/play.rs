use std::time::Duration;

use humantime::format_duration;

use serenity_feature_only::builder::CreateEmbed;
use songbird::input::{Compose, Input, YoutubeDl};

use crate::{
	commands::base_embed,
	metadata::AuxMetadataKey,
	shared::{enter_vc, is_url},
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

	enter_vc(ctx, true, |conn, ctx| async move {
		let reqwest = ctx.data().reqwest.clone();
		let time = get_time(query.as_str());
		let mut source = if is_url(&query) {
			YoutubeDl::new(reqwest, query)
		} else {
			YoutubeDl::new(reqwest, format!("ytsearch1:{}", query))
		};

		let channel_id = ctx.channel_id();
		let metadata = source.aux_metadata().await?;
		let mut lock = conn.lock().await;
		let handle = lock.enqueue_input(Input::from(source)).await;

		drop(lock);

		// To provent the bot from earaping people
		let _ = handle.set_volume(0.5);
		if let Some(time) = time {
			let _ = handle.seek(time);
		}

		let mut typemap = handle.typemap().write().await;
		typemap.insert::<AuxMetadataKey>(metadata.clone());

		ctx.send({
			let builder = poise::CreateReply::default();

			builder.embed(
				base_embed(CreateEmbed::default())
					.title(format!("Queueing audio in <#{channel_id}>"))
					.field(
						"Title",
						if let Some(title) = metadata.title {
							title
						} else {
							metadata.track.unwrap()
						},
						false,
					)
					.field(
						"Duration",
						if let Some(duration) = metadata.duration {
							format_duration(duration).to_string()
						} else {
							"∞".to_owned()
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

		Ok(())
	})
	.await
}

fn get_time(url: &str) -> Option<Duration> {
	let url = url.split('?').collect::<Vec<&str>>();
	let mut time = None;

	if url.len() > 1 {
		let mut params = url[1].split('&').collect::<Vec<&str>>();

		params.retain(|param| param.starts_with("t="));

		if !params.is_empty() {
			let time_str = params[0].split('=').collect::<Vec<&str>>()[1];

			if let Ok(time_parsed) = time_str.parse::<u64>() {
				return Some(Duration::new(time_parsed, 0));
			};

			time = Some(Duration::new(0, 0));
		}
	}

	time
}
