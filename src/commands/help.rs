use crate::{FloopyContext, FloopyError};

#[poise::command(
	slash_command,
	prefix_command,
	rename = "help",
	track_edits,
	aliases("h")
)]
pub async fn command(
	ctx: FloopyContext<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "poise::builtins::autocomplete_command"]
	command: Option<String>,
) -> Result<(), FloopyError> {
	let bottom_text = format!(
		"\
Type {:?}help command for more info on a command.
You can edit your message to the bot and the bot will edit its response.",
		ctx.prefix(),
	);

	let config = poise::builtins::HelpConfiguration {
		extra_text_at_bottom: &bottom_text.as_str(),
		..Default::default()
	};

	poise::builtins::help(ctx, command.as_deref(), config).await?;
	Ok(())
}
