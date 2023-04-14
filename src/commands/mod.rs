use crate::{FloopyContext, FloopyError};

pub mod help;
pub mod ping;
pub mod player;

#[poise::command(prefix_command, slash_command, ephemeral = true)]
pub async fn register(ctx: FloopyContext<'_>) -> Result<(), FloopyError> {
	print!("Registering commands... ");
	poise::builtins::register_application_commands_buttons(ctx).await?;
	Ok(())
}
