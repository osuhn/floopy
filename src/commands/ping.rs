use crate::{FloopyContext, FloopyError};

#[poise::command(slash_command)]
pub async fn ping(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	let initial = ctx.created_at();
	let reply = ctx.send(|f| f.content("Pong!")).await?;
	let elapsed = reply.message().await.unwrap().timestamp - initial;

	reply
		.edit(ctx, |m| {
			m.content(format!("Pong! Took {}ms", elapsed.as_millis()))
		})
		.await?;

	Ok(())
}
