use crate::structs::{Command, CommandResult, Context};
use poise::serenity_prelude as serenity;
use serenity::{builder::CreateEmbed, model::Color};

pub mod help;
pub mod ping;
pub mod player;
pub mod system;

#[poise::command(prefix_command, slash_command, ephemeral = true, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> CommandResult {
	print!("Registering commands... ");
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}

pub fn base_embed(e: CreateEmbed) -> CreateEmbed {
	e.color::<Color>(Color::BLURPLE)
}

pub fn commands() -> Vec<Command> {
	[
		register(),
		system::command(),
		ping::command(),
		help::command(),
	]
	.into_iter()
	.chain(player::commands())
	.collect()
}
