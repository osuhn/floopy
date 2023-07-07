pub use anyhow::{Error, Result};
use std::sync::Arc;

#[derive(Clone)]
pub struct Data(pub Arc<DataInner>);

impl std::ops::Deref for Data {
	type Target = DataInner;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub struct DataInner {
	pub songbird: Arc<songbird::Songbird>,
	pub reqwest: reqwest::Client,
	pub ytextract: ytextract::Client,
}

pub type Command = poise::Command<Data, CommandError>;
pub type Context<'a> = poise::Context<'a, Data, CommandError>;
// pub type PrefixContext<'a> = poise::PrefixContext<'a, Data, CommandError>;
// pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, CommandError>;

pub type CommandError = Error;
pub type CommandResult<E = Error> = Result<(), E>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, CommandError>;
