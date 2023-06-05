use crate::{
	shared::{get_conn, leave_channel},
	FloopyContext, FloopyError,
};

#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "leave",
	guild_only,
	aliases("stop")
)]
pub async fn command(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	ctx.defer().await?;

	let conn = get_conn(&ctx).await?;

	if !conn.lock().await.queue().is_empty() {
		let _ = conn.lock().await.queue().stop();
	}

	match leave_channel(&ctx).await {
		Ok(_) => {
			ctx.send(|r| r.content("I left the voice channel")).await?;

			return Ok(());
		}

		Err(_) => {
			ctx.send(|r| r.content("An error occurred while trying to leave channel."))
				.await?;

			return Err(FloopyError::from("error"));
		}
	}
}
