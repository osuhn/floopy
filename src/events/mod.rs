use poise::serenity_prelude as serenity;
use serenity::FullEvent as Event;

mod ready;
pub mod songbird;

use crate::structs::{FrameworkContext, Result};
use ready::*;

pub async fn listen(framework_ctx: FrameworkContext<'_>, event: &Event) -> Result<()> {
	match event {
		Event::Ready {
			ctx,
			data_about_bot,
		} => ready(framework_ctx, ctx, data_about_bot).await,

		_ => Ok(()),
	}
}
