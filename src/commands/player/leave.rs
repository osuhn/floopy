use crate::{shared::{join_channel, leave_channel}, FloopyContext, FloopyError};

#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "skip",
	guild_only,
	aliases("s")
)]
pub async fn command(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	ctx.defer().await?;

	let (_guild_id, _channel_id, conn, _manager) = join_channel(&ctx).await?;

	let _ = conn.lock().await.queue().stop();
	
	match leave_channel(&ctx).await {
        Ok() => {
            ctx.send(|r| r.content("I left the voice channel")).await?;
            
            return Ok(())
        }
        
        Err(e) => {
            ctx.send(|r| r.content("An error occurred while trying to leave channel.")).await?;
            
            return Err(FloopyError::from("error"))
        }
    }
}
