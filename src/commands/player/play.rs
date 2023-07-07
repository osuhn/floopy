use futures::StreamExt;
use humantime::format_duration;
use serde::{Deserialize, Serialize};
use serenity_feature_only::builder::CreateEmbed;
use songbird::input::{Compose, Input, YoutubeDl};
use std::time::Duration;
use ytextract::playlist::Id;

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
		let ytextract = ctx.data().ytextract.clone();
		let time = get_time(query.as_str());

		let mut sources = if is_url(&query) {
			if let Some(playlist_id) = get_playlist_id(&query) {
				fetch_playlist(playlist_id, &ytextract, &reqwest).await
			} else {
				vec![YoutubeDl::new(reqwest, query)]
			}
		} else {
			vec![YoutubeDl::new(reqwest, format!("ytsearch1:{}", query))]
		};

		let channel_id = ctx.channel_id();
		let metadata = first(&mut sources).unwrap().aux_metadata().await?;
		let mut lock = conn.lock().await;

		for source in sources {
			let handle = lock.enqueue_input(Input::from(source.to_owned())).await;
			let metadata = source.clone().aux_metadata().await?;

			// To provent the bot from earaping people
			let _ = handle.set_volume(0.5);
			if let Some(time) = time {
				let _ = handle.seek(time);
			}

			let mut typemap = handle.typemap().write().await;
			typemap.insert::<AuxMetadataKey>(metadata.clone());
		}

		drop(lock);

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Output {
	pub url: String,
	ie_key: String,
	#[serde(default)]
	pub title: Option<String>,
	#[serde(default)]
	pub channel: Option<String>,
	_type: String,
}

impl Output {
	#[allow(dead_code)]
	pub fn is_playable(&self) -> bool {
		self._type == "url" && !self.is_playlist()
	}

	#[allow(dead_code)]
	pub fn is_playlist(&self) -> bool {
		self.ie_key == "YoutubePlaylist" || self.ie_key == "YoutubeTab"
	}
}

fn first<T>(v: &mut [T]) -> Option<&mut T> {
	v.first_mut()
}

/// Get the playlist ID from the given URL.
fn get_playlist_id(url: &str) -> Option<Id> {
	let mut params = url.split('&').collect::<Vec<&str>>();

	params.retain(|&x| x.starts_with("list="));

	params.first().map(|x| match x[5..].parse() {
		Ok(id) => id,
		Err(_) => todo!(),
	})
}

async fn fetch_playlist(
	id: Id,
	ytextract: &ytextract::Client,
	reqwest: &reqwest::Client,
) -> Vec<YoutubeDl> {
	println!("fetching playlist {}", id);
	let playlist = ytextract.playlist(id).await.unwrap();

	let videos = playlist.videos();

	futures::pin_mut!(videos);

	videos
		.map(|video| match video {
			Ok(video) => {
				println!("fetching video {}", video.id());
				YoutubeDl::new(
					reqwest.clone(),
					format!("https://www.youtube.com/watch?v={}", video.id().to_string()),
				)
			}

			Err(_) => todo!(),
		})
		.collect::<Vec<_>>()
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
