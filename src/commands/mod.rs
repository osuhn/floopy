use crate::{FloopyContext, FloopyError};
use serenity::{builder::CreateEmbed, utils::Color};

pub mod help;
pub mod system;
pub mod ping;
pub mod player;

#[poise::command(prefix_command, slash_command, ephemeral = true, hide_in_help)]
pub async fn register(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	print!("Registering commands... ");
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}

pub fn base_embed(e: &mut CreateEmbed) -> &mut CreateEmbed {
	e.color(Color::BLURPLE)
}
