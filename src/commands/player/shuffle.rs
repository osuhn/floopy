use crate::{
	shared::queue_modify,
	structs::{CommandResult, Context},
};

use rand::seq::SliceRandom;

/// Shuffle the queue.
#[poise::command(
	prefix_command,
	slash_command,
	ephemeral = true,
	rename = "shuffle",
	guild_only,
	aliases("shuf", "sh"),
	category = "Player",
	member_cooldown = 5
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	queue_modify(ctx, |x| {
		let slice = x.make_contiguous();
		slice[1..].shuffle(&mut rand::thread_rng());
		"Shuffled the queue!".into()
	})
	.await
}
