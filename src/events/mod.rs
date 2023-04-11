use std::sync::Arc;

use poise::serenity_prelude::{self as serenity, Mutex, RwLock, ShardManager, UserId};
use tracing::info;

use crate::{FloopyData, FloopyError};

pub struct Handler<T> {
	options: poise::FrameworkOptions<T, FloopyError>,
	data: T,
	bot_id: RwLock<Option<UserId>>,
	shard_manager: RwLock<Option<Arc<Mutex<ShardManager>>>>,
}

// Custom handler to dispatch poise events
impl<T: Send + Sync> Handler<T> {
	pub fn new(options: poise::FrameworkOptions<T, FloopyError>, data: T) -> Self {
		Self {
			options,
			data,
			shard_manager: RwLock::new(None),
			bot_id: RwLock::new(None),
		}
	}

	pub async fn set_shard_manager(&self, shard_manager: Arc<Mutex<serenity::ShardManager>>) {
		*self.shard_manager.write().await = Some(shard_manager);
	}

	async fn dispatch_poise_event(&self, ctx: &serenity::Context, event: &poise::Event<'_>) {
		let framework_data = poise::FrameworkContext {
			bot_id: self.bot_id.read().await.unwrap(),
			options: &self.options,
			user_data: &self.data,
			shard_manager: &(*self.shard_manager.read().await).clone().unwrap(), /* Shard manager can be read between all poise events without locks */
		};
		poise::dispatch_event(framework_data, ctx, event).await;
	}
}

// Manually dispatch events from serenity to poise
#[serenity::async_trait]
impl serenity::EventHandler for Handler<Arc<RwLock<FloopyData>>> {
	async fn message_update(
		&self,
		ctx: serenity::Context,
		old_if_available: Option<serenity::Message>,
		new: Option<serenity::Message>,
		event: serenity::MessageUpdateEvent,
	) {
		self.dispatch_poise_event(
			&ctx,
			&poise::Event::MessageUpdate {
				old_if_available,
				new,
				event,
			},
		)
		.await;
	}

	async fn message(&self, ctx: serenity::Context, new_message: serenity::Message) {
		self.dispatch_poise_event(&ctx, &poise::Event::Message { new_message })
			.await;
	}

	async fn ready(&self, _ctx: serenity::Context, ready: serenity::Ready) {
		*self.bot_id.write().await = Some(ready.user.id);

		info!("{} is ready", ready.user.name)
	}

	async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction) {
		self.dispatch_poise_event(&ctx, &poise::Event::InteractionCreate { interaction })
			.await;
	}
}
