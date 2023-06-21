use std::{future::Future, sync::Arc};

use poise::serenity_prelude as serenity;
use serenity::model::id::{ChannelId, GuildId};
use songbird::Call;
use tokio::sync::Mutex;

use crate::{
	error::{AlreadyInVoiceChannelError, Error, NoSongbirdError},
	events::songbird::{EndLeaver, ErrorHandler},
	structs::{CommandResult, Context},
};

use super::error::{NotInGuildError, NotInVoiceChannelError};

pub async fn leave_channel(ctx: &Context<'_>) -> Result<(GuildId, ChannelId), Error> {
	let guild_id = ctx.guild_id().ok_or(NotInGuildError)?;
	let manager = songbird::get(ctx.discord()).await.ok_or(NotInGuildError)?;

	let channel_id = if let Some(conn) = manager.get(guild_id) {
		let channel_id = {
			conn.lock()
				.await
				.current_channel()
				.ok_or(NotInVoiceChannelError)
		};

		manager.remove(guild_id).await?;

		channel_id?
	} else {
		return Err(NotInVoiceChannelError.into());
	};

	Ok((guild_id, ChannelId(channel_id.0)))
}

pub async fn try_join(ctx: Context<'_>, must_join: bool) -> Result<Arc<Mutex<Call>>, Error> {
	let guild = ctx.guild_id().unwrap();
	let user = ctx.author().id;
	let manager = songbird::get(ctx.discord())
		.await
		.ok_or(NoSongbirdError)?
		.clone();

	if let Some(call) = manager.get(guild) {
		if must_join {
			return Err(AlreadyInVoiceChannelError.into());
		} else {
			return Ok(call);
		}
	}

	let channel_id = guild
		.to_guild_cached(ctx.discord())
		.unwrap()
		.voice_states
		.get(&user)
		.and_then(|voice_state| voice_state.channel_id);

	let connect_to = match channel_id {
		Some(channel) => channel,
		None => return Err(NotInVoiceChannelError.into()),
	};

	let handler = manager
		.join(guild, connect_to)
		.await
		.map_err(|_x| NoSongbirdError)?;

	handler.lock().await.add_global_event(
		songbird::Event::Track(songbird::TrackEvent::Error),
		ErrorHandler,
	);

	handler.lock().await.add_global_event(
		songbird::Event::Track(songbird::TrackEvent::End),
		EndLeaver {
			manager,
			guild_id: guild,
		},
	);

	Ok(handler)
}

pub async fn enter_vc<
	'a,
	F: FnOnce(Arc<Mutex<Call>>, Context<'a>) -> T + 'a,
	T: Future<Output = CommandResult> + 'a,
>(
	ctx: Context<'a>,
	autojoin: bool,
	f: F,
) -> CommandResult {
	let guild_id = ctx.guild_id().unwrap();

	let manager = songbird::get(ctx.discord())
		.await
		.ok_or(NoSongbirdError)?
		.clone();

	let handler_lock = if autojoin {
		match try_join(ctx, false).await {
			Ok(x) => x,
			Err(e) => {
				ctx.say(format!("failed to autojoin: {e:?}")).await?;

				return Ok(());
			}
		}
	} else {
		match manager.get(guild_id) {
			Some(handler) => handler,
			None => {
				ctx.say("Not in a voice channel").await?;
				return Ok(());
			}
		}
	};

	f(handler_lock, ctx).await
}
