use crate::{FloopyContext, FloopyError};
use songbird;

#[poise::command(prefix_command, slash_command, ephemeral = true)]
pub async fn play(
	ctx: FloopyContext<'_>,
	#[description = "The song to play"] query: String,
) -> Result<(), FloopyError> {
	let guild_id = ctx.guild_id().unwrap();

	let manager = songbird::get(ctx.serenity_context()).await.unwrap().clone();
	let lava_client = ctx.data().read().await.lavalink.clone();

	if manager.get(guild_id).is_none() {
		let guild = ctx.guild().unwrap();

		let channel_id = guild
			.voice_states
			.get(&ctx.author().id)
			.and_then(|voice_state| voice_state.channel_id);

		let connect_to = match channel_id {
			Some(channel) => channel,
			None => {
				ctx.send(|f| f.content("Join a voice channel.")).await?;

				return Ok(());
			}
		};

		let (_, handler) = manager.join_gateway(guild_id, connect_to).await;

		match handler {
			Ok(connection_info) => {
				lava_client
					.create_session_with_songbird(&connection_info)
					.await?;

				ctx.send(|f| f.content(format!("Joined {}", connect_to.to_string())))
					.await?;
			}
			Err(why) => {
				ctx.send(|f| f.content(format!("Error joining the channel: {}", why)))
					.await?;
			}
		}
	}

	let query_info = lava_client.auto_search_tracks(&query).await?;

	if query_info.tracks.is_empty() {
		ctx.send(|f| f.content("No tracks found")).await?;
		return Ok(());
	}

	if let Err(why) = &lava_client
		.play(guild_id, query_info.tracks[0].clone())
		.queue()
		.await
	{
		eprintln!("{}", why);
		return Ok(());
	};

	ctx.send(|f| {
		f.content(format!(
			"Added to queue: {}",
			query_info.tracks[0].info.as_ref().unwrap().title
		))
	})
	.await?;

	Ok(())
}
