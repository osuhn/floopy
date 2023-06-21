use poise::serenity_prelude;
use songbird::error::JoinError as SongbirdJoinError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Not in a guild")]
pub struct NotInGuildError;

#[derive(Debug, Error)]
#[error("Not in a voice channel")]
pub struct NotInVoiceChannelError;

#[derive(Debug, Error)]
#[error("Voice channel has no ID")]
pub struct NoVoiceChannelIdError;

#[derive(Debug, Error)]
#[error("Songbird hasn't been initialized (this is a bug)")]
pub struct NoSongbirdError;

#[derive(Debug, Error)]
#[error("Already in a voice channel")]
pub struct AlreadyInVoiceChannelError;

#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Serenity(#[from] serenity_prelude::Error),

	#[error(transparent)]
	NotInGuild(#[from] NotInGuildError),

	#[error(transparent)]
	NotInVoiceChannel(#[from] NotInVoiceChannelError),

	#[error(transparent)]
	NoVoiceChannelId(#[from] NoVoiceChannelIdError),

	#[error(transparent)]
	NoSongbird(#[from] NoSongbirdError),

	#[error(transparent)]
	SongbirdJoin(#[from] SongbirdJoinError),

	#[error(transparent)]
	AlreadyInVoiceChannel(#[from] AlreadyInVoiceChannelError),
}
