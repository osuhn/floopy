use crate::structs::{Command, CommandResult, Context};
use poise::serenity_prelude as serenity;
use serenity::{builder::CreateEmbed, model::Color};

pub mod bot;
pub mod player;

#[poise::command(prefix_command, slash_command, ephemeral = true, hide_in_help)]
pub async fn register(ctx: Context<'_>) -> CommandResult {
	print!("Registering commands... ");
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}

pub fn base_embed(e: CreateEmbed) -> CreateEmbed {
	e.color::<Color>(Color::BLURPLE)
}

pub fn error_embed(e: CreateEmbed) -> CreateEmbed {
	e.color::<Color>(Color::RED).title("Error!")
}

pub fn commands() -> Vec<Command> {
	[register()]
		.into_iter()
		.chain(player::commands())
		.chain(bot::commands())
		.collect()
}
