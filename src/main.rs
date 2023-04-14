#![warn(clippy::str_to_string)]

mod commands;
mod events;

use commands::*;
use events::{Handler, LavalinkHandler};
use lavalink_rs::LavalinkClient;
use poise::serenity_prelude::{self as serenity, RwLock};
use std::{
	collections::{HashMap, HashSet},
	env::var,
	sync::Arc,
};

type FloopyError = Box<dyn std::error::Error + Send + Sync>;
type FloopyContext<'a> = poise::Context<'a, Arc<RwLock<FloopyData>>, FloopyError>;

impl serenity::TypeMapKey for FloopyData {
	type Value = Arc<RwLock<FloopyData>>;
}

pub struct FloopyData {
	pub queue: HashMap<String, String>,
	pub lavalink: LavalinkClient,
}

#[tokio::main]
async fn main() {
	let token = var("DISCORD_TOKEN").expect("Expected a token in the environment");

	// load .env file
	dotenv::dotenv().ok();

	// init logger
	tracing_subscriber::fmt::init();

	let mut owners = HashSet::new();
	owners.insert(serenity::UserId(462780441594822687));

	let mut commands = vec![register(), help::help(), ping::ping(), player::play::play()];
	poise::set_qualified_names(&mut commands);

	let http = serenity::Http::new(&token);

	let bot_id = match http.get_current_application_info().await {
		Ok(info) => info.id,
		Err(why) => panic!("Could not access application info: {:?}", why),
	};

	let lava_client = LavalinkClient::builder(bot_id.0)
		.set_host(var("LAVALINK_HOST").expect("Expected a host in the environment"))
		.set_password(var("LAVALINK_PASSWORD").expect("Expected a password in the environment"))
		.set_port(
			var("LAVALINK_PORT")
				.expect("Expected a port in the environment")
				.parse::<u16>()
				.expect("Expected a valid port"),
		)
		.build(LavalinkHandler)
		.await
		.expect("Failed to build Lavalink client");

	let data = Arc::new(RwLock::new(FloopyData {
		queue: HashMap::new(),
		lavalink: lava_client,
	}));

	let handler = Arc::new(Handler::new(
		poise::FrameworkOptions {
			owners,
			commands,
			prefix_options: poise::PrefixFrameworkOptions {
				prefix: Some("~".into()),
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
	.event_handler_arc(handler.clone())
	.await
	.unwrap();

	client.data.write().await.insert::<FloopyData>(data);

	handler
		.set_shard_manager(client.shard_manager.clone())
		.await;

	client.start().await.unwrap();
}
