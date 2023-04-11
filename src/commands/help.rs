use crate::{FloopyContext, FloopyError};

#[poise::command(slash_command)]
pub async fn help(
	ctx: FloopyContext<'_>,
	#[description = "Specific command to show help about"]
	#[autocomplete = "poise::builtins::autocomplete_command"]
	command: Option<String>,
) -> Result<(), FloopyError> {
	poise::builtins::help(
		ctx,
		command.as_deref(),
		poise::builtins::HelpConfiguration {
			extra_text_at_bottom: "\
This is an example bot made to showcase features of my custom Discord bot framework",
			show_context_menu_commands: true,
			..Default::default()
		},
	)
	.await?;
	Ok(())
}
