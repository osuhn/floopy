#![warn(clippy::str_to_string)]
#![allow(stable_features)]
#![feature(let_chains, async_closure)]

mod commands;
mod constants;
mod error;
mod events;
mod metadata;
mod shared;
mod structs;

use anyhow::Result;

use commands::*;
use poise::serenity_prelude as serenity;
use songbird::SerenityInit;
use std::{collections::HashSet, env::var, sync::Arc};
use structs::{Data, DataInner};
use tracing::log::warn;

fn main() -> Result<()> {
	std::env::set_var("RUST_LIB_BACKTRACE", "1");
	dotenv::dotenv().ok();
	tracing_subscriber::fmt::init();

	tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()?
		.block_on(_main())
}

async fn _main() -> Result<()> {
	let mut owners = HashSet::new();
	owners.insert(serenity::UserId::new(462780441594822687));

	let reqwest = reqwest::Client::new();
	let ytextract = ytextract::Client::new();

	let data = Data(Arc::new(DataInner {
		songbird: songbird::Songbird::serenity(),
		reqwest,
		ytextract,
	}));

	let framework_options = poise::FrameworkOptions {
		owners,
		commands: commands::commands(),
		listener: |event, ctx, _| Box::pin(events::listen(ctx, event)),
		prefix_options: poise::PrefixFrameworkOptions {
			prefix: Some(var("DISCORD_PREFIX").unwrap_or(";".to_owned())),
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
		allowed_mentions: Some(
			serenity::CreateAllowedMentions::default()
				.replied_user(true)
				.all_users(true),
		),
		..poise::FrameworkOptions::default()
	};

	let token = var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

	let intents = serenity::GatewayIntents::non_privileged()
		| serenity::GatewayIntents::MESSAGE_CONTENT
		| serenity::GatewayIntents::GUILD_MESSAGES
		| serenity::GatewayIntents::GUILD_INTEGRATIONS;

	let mut client = serenity::Client::builder(token, intents)
		.register_songbird()
		.framework(poise::Framework::new(framework_options, |_, _, _| {
			Box::pin(async { Ok(data) })
		}))
		.await?;

	let shard_manager = client.shard_manager.clone();

	tokio::spawn(async move {
		#[cfg(unix)]
		{
			use tokio::signal::unix as signal;

			let [mut s1, mut s2, mut s3] = [
				signal::signal(signal::SignalKind::hangup()).unwrap(),
				signal::signal(signal::SignalKind::interrupt()).unwrap(),
				signal::signal(signal::SignalKind::terminate()).unwrap(),
			];

			tokio::select!(
				v = s1.recv() => v.unwrap(),
				v = s2.recv() => v.unwrap(),
				v = s3.recv() => v.unwrap(),
			);
		}
		#[cfg(windows)]
		{
			let (mut s1, mut s2) = (
				tokio::signal::windows::ctrl_c().unwrap(),
				tokio::signal::windows::ctrl_break().unwrap(),
			);

			tokio::select!(
				v = s1.recv() => v.unwrap(),
				v = s2.recv() => v.unwrap(),
			);
		}

		warn!("Recieved control C and shutting down.");
		shard_manager.lock().await.shutdown_all().await;
	});

	let _ = client
		.start_autosharded()
		.await
		.map_err(Into::<structs::Error>::into);

	Ok(())
}
