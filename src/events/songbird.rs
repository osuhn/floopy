use songbird::EventContext;
use std::sync::Arc;

use poise::serenity_prelude as serenity;
use songbird::{tracks::PlayMode, Songbird};

pub struct ErrorHandler;

#[serenity::async_trait]
impl songbird::EventHandler for ErrorHandler {
	async fn act(&self, ctx: &EventContext<'_>) -> Option<songbird::Event> {
		if let EventContext::Track(e) = ctx {
			for t in *e {
				if let PlayMode::Errored(e) = &t.0.playing {
					log::warn!("Error occurred while playing track: {:?}", e);
				}
			}
		}
		None
	}
}

pub struct EndLeaver {
	pub manager: Arc<Songbird>,
	pub guild_id: serenity::GuildId,
}

#[serenity::async_trait]
impl songbird::EventHandler for EndLeaver {
	async fn act(&self, _ctx: &EventContext<'_>) -> Option<songbird::Event> {
		if let Some(conn) = self.manager.get(self.guild_id) {
			let should_remove = conn.lock().await.queue().is_empty();
			if should_remove {
				if let Err(err) = self.manager.leave(self.guild_id).await {
					eprintln!("Failed to leave after track end: {err}");
				}
			}
		}
		None
	}
}
