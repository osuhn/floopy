use crate::structs::{CommandResult, Context};

/// Ping the bot.
#[poise::command(
	slash_command,
	prefix_command,
	rename = "ping",
	aliases("latency"),
	category = "Bot"
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	let initial = ctx.created_at().timestamp_millis();
	let reply = ctx
		.send(poise::CreateReply::default().content("Pong!"))
		.await?;
	let end = reply.message().await.unwrap().timestamp.timestamp_millis();
	let elapsed = end - initial;

	reply
		.edit(
			ctx,
			poise::CreateReply::default().content(format!("Pong! Took {}ms", elapsed)),
		)
		.await?;

	Ok(())
}
