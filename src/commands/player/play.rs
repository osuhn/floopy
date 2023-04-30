use std::sync::Arc;

use humantime::format_duration;

use poise::serenity_prelude::Color;
use serenity::{async_trait, model::id::GuildId};
use songbird::{
	input::{Input, Restartable},
	Event, EventContext, EventHandler as VoiceEventHandler, Songbird, TrackEvent,
};

use crate::{shared::join_channel, FloopyContext, FloopyError};
use url::Url;

#[poise::command(prefix_command, slash_command, ephemeral = true)]
pub async fn play(
	ctx: FloopyContext<'_>,
	#[description = "The song to play"]
	#[rest]
	query: String,
) -> Result<(), FloopyError> {
	ctx.defer().await?;

	let source: Input = if let Ok(url) = Url::parse(&query) {
		Restartable::ytdl(url, true).await?.into()
	} else {
		Restartable::ytdl_search(query, true).await?.into()
	};

	let (guild_id, channel_id, conn, manager) = join_channel(&ctx).await?;

	let metadata = source.metadata.clone();

	let handle = conn.lock().await.enqueue_source(source);

	let _ = handle.add_event(
		Event::Track(TrackEvent::End),
		EndLeaver { manager, guild_id },
	);

	ctx.send(|r| {
		r.embed(|e| {
			e.color(Color::BLURPLE)
				.title(format!("Queueing audio in <#{channel_id}>"));

			if let Some(title) = &metadata.title {
				e.field("Title", title, false);
			} else if let Some(track) = &metadata.track {
				e.field("Title", track, false);
			}

			if let Some(duration) = &metadata.duration {
				e.field("Duration", format_duration(*duration), true);
			}

			if let Some(source_url) = &metadata.source_url {
				e.field("Source", format!("[Open original]({source_url})"), true);
			}

			if let Some(thumbnail) = &metadata.thumbnail {
				e.thumbnail(thumbnail);
			}

			e
		})
	})
	.await?;

	return Ok(());
}

struct EndLeaver {
	pub manager: Arc<Songbird>,
	pub guild_id: GuildId,
}

#[async_trait]
impl VoiceEventHandler for EndLeaver {
	async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
		if let Some(conn) = self.manager.get(self.guild_id) {
			let should_remove = conn.lock().await.queue().is_empty();
			if should_remove {
				if let Err(err) = self.manager.remove(self.guild_id).await {
					eprintln!("Failed to leave after track end: {err}");
				}
			}
		}
		None
	}
}
