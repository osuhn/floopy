use std::time::SystemTime;

use crate::{FloopyContext, FloopyError};

#[poise::command(slash_command)]
pub async fn ping(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	let initial = SystemTime::now();

	let reply = ctx.send(|f| f.content("Pong!")).await?;

	let elapsed = initial.elapsed().unwrap();

	reply
		.edit(ctx, |m| {
			m.content(format!("Pong! Took {}ms", elapsed.as_millis()))
		})
		.await?;

	Ok(())
}
