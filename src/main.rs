#![warn(clippy::str_to_string)]

mod commands;
mod error;
mod events;
mod shared;

use commands::*;
use events::Handler;
use poise::serenity_prelude::{self as serenity, RwLock};
use songbird::SerenityInit;
use std::{collections::HashSet, env::var, sync::Arc};

pub use self::error::Error;

pub type FloopyError = Box<dyn std::error::Error + Send + Sync>;
pub type FloopyContext<'a> = poise::Context<'a, Arc<RwLock<FloopyData>>, FloopyError>;
pub type Result<R> = core::result::Result<R, Error>;

impl serenity::TypeMapKey for FloopyData {
	type Value = Arc<RwLock<FloopyData>>;
}

pub struct FloopyData {
	pub songbird: Arc<songbird::Songbird>,
}

#[tokio::main]
async fn main() {
	// load .env file
	dotenv::dotenv().ok();

	// init logger
	tracing_subscriber::fmt::init();

	let mut owners = HashSet::new();
	owners.insert(serenity::UserId(462780441594822687));

	let mut commands = vec![
		register(),
		help::command(),
		ping::command(),
		player::play::command(),
		player::skip::command(),
		player::leave::command(),
		player::queue::command(),
		mem_usage::command(),
	];
	poise::set_qualified_names(&mut commands);

	let data = Arc::new(RwLock::new(FloopyData {
		songbird: songbird::Songbird::serenity(),
	}));

	let handler = Arc::new(Handler::new(
		poise::FrameworkOptions {
			owners,
			commands,
			prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some(":".into()),
				edit_tracker: Some(poise::EditTracker::for_timespan(
					std::time::Duration::from_secs(3600),
				)),
				case_insensitive_commands: true,
				..Default::default()
			},
			on_error: |error| {
				Box::pin(async {
					poise::samples::on_error(error)
						.await
						.unwrap_or_else(|error| tracing::error!("{}", error))
				})
			},
			event_handler: |_ctx, event, _framework, _data| {
				Box::pin(async move {
					tracing::trace!("{:?}", event.name());
					Ok(())
				})
			},
			skip_checks_for_owners: true,
			..Default::default()
		},
		data.clone(),
	));

	let mut client = serenity::Client::builder(
		var("DISCORD_TOKEN")
			.expect("Missing `DISCORD_TOKEN` env var, see README for more information."),
		serenity::GatewayIntents::non_privileged()
			| serenity::GatewayIntents::MESSAGE_CONTENT
			| serenity::GatewayIntents::GUILD_MESSAGES
			| serenity::GatewayIntents::GUILD_INTEGRATIONS,
	)
	.register_songbird()
	.event_handler_arc(handler.clone())
	.await
	.unwrap();

	client.data.write().await.insert::<FloopyData>(data);

	handler
		.set_shard_manager(client.shard_manager.clone())
		.await;

	client.start().await.unwrap();
}
