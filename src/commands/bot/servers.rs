use crate::structs::{CommandResult, Context};

/// List the servers the bot is in.
#[poise::command(
	slash_command,
	prefix_command,
	rename = "servers",
	track_edits,
	aliases("h"),
	category = "Bot"
)]
pub async fn command(ctx: Context<'_>) -> CommandResult {
	poise::builtins::servers(ctx).await?;
	Ok(())
}
