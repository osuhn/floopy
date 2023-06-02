use crate::{FloopyContext, FloopyError};

#[poise::command(slash_command, prefix_command, rename = "ping")]
pub async fn command(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	let initial = ctx.created_at().timestamp_millis();
	let reply = ctx.send(|f| f.content("Pong!")).await?;
	let end = reply.message().await.unwrap().timestamp.timestamp_millis();
	let elapsed = end - initial;

	reply
		.edit(ctx, |m| m.content(format!("Pong! Took {}ms", elapsed)))
		.await?;

	Ok(())
}
