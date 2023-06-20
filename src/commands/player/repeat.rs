use serenity_feature_only::builder::CreateEmbed;

use crate::{
	commands::error_embed,
	shared::enter_vc,
	structs::{CommandResult, Context},
};

/// Leaves the current voice channel.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "loop",
	guild_only,
	aliases("repeat", "looping", "repeating"),
	category = "Player",
	member_cooldown = 5
)]
pub async fn command(
	ctx: Context<'_>,
	#[description = "Times to loop the current song. If not specified, loops indefinitely."]
	#[min = 1]
	#[max = 100]
	#[lazy]
	times: Option<usize>,
) -> CommandResult {
	ctx.defer().await?;

	enter_vc(ctx, false, |conn, ctx| async move {
		if conn.lock().await.queue().is_empty() {
			ctx.send(poise::CreateReply::default().embed(
				error_embed(CreateEmbed::default()).description("There is nothing to loop!"),
			))
			.await?;
			return Ok(());
		}

		if times.is_some() {
			let _ = conn
				.lock()
				.await
				.queue()
				.current()
				.unwrap()
				.loop_for(times.unwrap());

			ctx.send(
				poise::CreateReply::default()
					.content(format!("Looping for {} times.", times.unwrap())),
			)
			.await?;

			return Ok(());
		}

		let _ = conn.lock().await.queue().current().unwrap().enable_loop();

		ctx.send(poise::CreateReply::default().content("Looping indefinitely."))
			.await?;

		Ok(())
	})
	.await
}
