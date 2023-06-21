use poise::serenity_prelude as serenity;

use crate::structs::{FrameworkContext, Result};

#[allow(clippy::explicit_auto_deref)]
pub async fn ready(
	_framework_ctx: FrameworkContext<'_>,
	_ctx: &serenity::Context,
	data_about_bot: &serenity::Ready,
) -> Result<()> {
	log::info!("{} is connected!", data_about_bot.user.name);

	Ok(())
}
